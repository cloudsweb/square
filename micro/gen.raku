use Template::Mustache;

my $worksapce = $*PROGRAM.parent;

say "working in $worksapce";

# run <ls -la>, $worksapce.add("dao/model").absolute;
say "go run cmd/gen/main.go";
run <git clean -fX dao>, cwd=>$worksapce;
# run <go mod tidy>, cwd=>$worksapce;
run <go run cmd/gen/main.go>, cwd=>$worksapce;

my %protobuf_type_map =
  "time.Time" => "google.protobuf.Timestamp",
  ;

class Table {
  has Str $.name;
  has %.names;
  has @.fields;

  multi method new($struct-name, $parse-tag, @fields) {
    my %names = $parse-tag => $struct-name;
    my $name = do given $parse-tag {
      when "go_orm" { $struct-name.subst(/ORM$/) }
      when "go_proto" { $struct-name }
      default { $struct-name }
    };
    self.bless(:$name, :%names, :@fields)
  }
}

class Field {
  has Int $.idx;
  has Str $.parse-tag;
  has Str $.ident;
  has Str $.type;
  has Bool $.optional;
  has %.names; # proto, go_orm, go_proto
  has %.types;
  has %.go-attrs; # gorm, json, protobuf, ...

  method TWEAK {
    $!optional //= %!types<go_orm> ?? %!types<go_orm>.starts-with('*') !! Bool;
  }

  method rawtype {
    my $gotype = %!types<go_orm>.subst(/^\*/);
    if %protobuf_type_map{$gotype} {
      %protobuf_type_map{$gotype}
    } else {
      $gotype
    }
  }
  method pos-idx {
    $!idx + 1
  }
  method optional-str {
    if $.optional {
      "optional"
    } else {
      "required"
    }
  }
  method name {
    if %!go-attrs<json> {
      %!go-attrs<json>.first
    } else {
      $.goname
    }
  }
}
grammar field_grammar {
  rule TOP { <.ws> <ident> <type> "`"<tags>"`" <comment>? }
  rule tags { <subtag> +  }
  rule comment { '//'.* }
  token subtag { <ident>':"'<expr>'"' }
  token expr { <-["]>+ }
  token ident { <[a..zA..Z]>\w* }
  token type { "*"?<[a..zA..Z]><[.\w]>* }
}

class Parser {
  has Int $.idx;
  has $.parse-tag = "go_orm"; # orm | protobuf

  method TOP ($/) {
    if !self.DEFINITE {
      return self.new.TOP($/);
    }
    my $name = $<ident>.Str;
    my $gotype = $<type>.Str;
    my %attrs = $<tags>.made;
    my $ident = %attrs<json> ?? %attrs<json>.first !! $name;
    my $type = %attrs<gorm><type> || $gotype;
    make Field.new(:$!idx, :$!parse-tag, :$ident, :$type, names=>%{ $.parse-tag => $name }, types=>%{ $.parse-tag => $gotype }, go-attrs=>%attrs)
  }
  method tags ($/) { make $<subtag>.map({ .made }).Hash }
  method subtag ($/) {
    my $name = $<ident>.Str;
    given $name {
      when 'gorm' { make $name => self.gorm($<expr>) }
      when 'json' { make $name => $<expr>.Str.split(',').list }
      default { make $name => $<expr>.Str }
    }
  }
  method gorm ($/) {
    my @rules;
    my %gorm;
    for $/.Str.split(';') -> $item {
      if $item.contains(':') {
        (my $k, my $v) = $item.split(':', 2);
        %gorm{$k} = $v;
      } else {
        @rules.push($item);
      }
    }
    if @rules {
      %gorm<rules> = @rules
    }
    make %gorm
  }
}

sub load_template($filename, Str:D :$lead_str="//",
  Str:D :$lead_add="$lead_str+ ", Str:D :$lead_sub="$lead_str~ ", Str:D :$lead_ignore="$lead_str- "
) {
  my @subs;
  my $ignore = Str;
  my $lines = gather for $filename.IO.lines {
    my $line = $_;
    my $line_trim = $line.trim;
    if $ignore {
      # say "'$ignore', '$line_trim'";
      given $ignore {
        when $line_trim { $ignore = Str }
        when $lead_ignore.trim { $ignore = Str; proceed }
        default {
          take $lead_ignore ~ $line # TODO: ident?
        }
      }
      next
    }
    if $line_trim.starts-with($lead_ignore) || $line_trim === $lead_ignore.trim {
      $ignore = $line_trim;
      next
    }
    if $line_trim.starts-with($lead_sub) {
      (my $replace, my $to) = $line_trim.subst($lead_sub, :x(1)).split('=', 2).map({ .trim });
      @subs.push($replace => $to);
      next
    }
    if $line_trim.starts-with($lead_add) {
      $line = $line.subst($lead_add, :x(1))
    }
    if @subs {
      # $line = reduce(-> $s, $t { $s.subst($t.key, $t.value) }, $line, | @subs);
      my %subs = @subs.Hash;
      my regex R { @(%subs.keys) };
      $line = S:g/<!ww><R><!ww>/%subs{$/}/ given $line;
      @subs := Array.new;
    }
    take $line.subst(/<<LL__(\w+)__TT>>/, {"\{\{ $0 \}\}"}, :g);
  }
  $lines.join("\n")
}

sub load_model($filename, $parse-tag) {
  my @file_lines = $filename.IO.lines;
  # TODO: check if only one model and matches filename
  my @result;
  my $struct_name = Str;
  my @fields;
  my $i = 0;
  for @file_lines -> $line {
    if $line.trim.starts-with('type ') {
      $struct_name = $line.match(rx:s/type <(\w+)> struct/).Str;
    }
    if $line.contains('`gorm:'|'`protobuf:') {
      @fields.push(field_grammar.parse($line, actions=>Parser.new(:idx($i++), :$parse-tag)).made)
    }
    if $line.trim.starts-with('}') {
      if $struct_name {
        @result.push(Table.new($struct_name, $parse-tag, @fields))
      }
      $struct_name = Str;
      $i = 0;
    }
  }
  @result
}

# get files in dao/model
# ["dao/model/users.gen.go".IO "dao/model/secrets.gen.go".IO "dao/model/posts.gen.go".IO]
my @model_files = $worksapce.add("dao/model").dir: test => { .ends-with(".gen.go") };
# {posts => "dao/model/posts.gen.go".IO, ...}
my %models = @model_files.map({ .basename.subst(/'.'gen'.'go$/) }) Z=> @model_files;
say "load models {%models.keys}";

my @structs = %models.values.map({ load_model($_, "go_orm") }).flat.sort({ $_.name });

# save proto
my $proto_template = load_template($worksapce.add("templates/LL__db__TT.proto"));
# say $template;
my $proto = Template::Mustache.render($proto_template, {
  go_package => './dao/model',
  proto_package => 'square.db',
  :@structs,
});
my $out_proto = "protos/db.proto";
$worksapce.add($out_proto).spurt($proto);
say "run protoc $out_proto";
run ("protoc", "--go_out=.", $worksapce.add($out_proto).Str), cwd=>$worksapce;

# convert
my @structs_pb = load_model($worksapce.add('dao/model/db.pb.go'), "go_proto").sort({ $_.name });
# say (@structs Z, @structs_pb).map({{ |$_[0], fields_pb => $_[1]<fields> }});
my $converter_template = load_template($worksapce.add("templates/LL__converter__TT.go"), lead_str=>'// ');
# say $converter_template;

run <go mod tidy>, cwd=>$worksapce;
