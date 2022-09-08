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

  method go_proto_name { %.names<go_proto> }
  method go_orm_name { %.names<go_orm> }
  method go_imports {
    my $result = gather for @.fields -> $field {
      take "google.golang.org/protobuf/types/known/timestamppb" if $field.types<go_proto>.contains("timestamppb.");
    }
    $result.flat.unique(:with(&[eq])).Seq
  }

  method push($b) {
    if $.name ne $b.name {
      say "table update: $.name != {$b.name}, ignore";
      return
    }
    say "udpate $.name";
    %!names = %( |$b.names, |%.names );
    (@!fields Z, $b.fields).flat.map: -> $u, $v { $u.push($v) };
    $(self)
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

  method name { $!ident }
  method go_proto_name { %.names<go_proto> }
  method go_orm_name { %.names<go_orm> }

  method go_convert($from-name, $from-type, $to-name, $to-type, :$indent) {
    my $src-expr = "m.{$from-name}";
    my $dst-expr = "to.{$to-name}";
    my $src-non-nil = "$src-expr != nil";
    my $src-is-pointer = $from-type.starts-with("*");
    my $dst-is-pointer = $to-type.starts-with("*");
    my $src-type = $from-type.subst(/^\*/);
    my $dst-type = $to-type.subst(/^\*/);
    my $stmt = "";
    my $comment = "";
    if $src-type ne $dst-type {
      $comment ~= "\n// transformer: $src-type => $dst-type";
      if $src-type eq "timestamppb.Timestamp" && $dst-type eq "time.Time" {
        $src-expr = "{$src-expr}.AsTime()";
        $src-is-pointer = False;
      } elsif $src-type eq "time.Time" && $dst-type eq "timestamppb.Timestamp" {
        if $src-is-pointer {
          $src-expr = "*$src-expr";
        }
        $src-expr = "timestamppb.New($src-expr)";
        $src-is-pointer = True;
      } else {
        $comment ~= "  TODO: unimplemented";
        say "unimplemented $src-type => $dst-type";
      }
    };
    if !$src-is-pointer && $dst-is-pointer {
      $src-expr = "&$src-expr";
    } elsif $src-is-pointer && !$dst-is-pointer {
      $src-expr = "*$src-expr";
    }
    $stmt ~= "$dst-expr = $src-expr";
    if $src-is-pointer && !$dst-is-pointer {
      $stmt = qq:to/CODE/
      if $src-non-nil \{
        $stmt
      \}
      CODE
    }
    my $result = qq:to/CODE/
    {$comment.trim}
    {$stmt.trim}
    CODE
    ;
    $result.trim.lines.join("\n" ~ $indent)
  }
  method go_convert_to_orm($from-name = $.names<go_proto>, $from-type = $.types<go_proto>, $to-name = $.names<go_orm>, $to-type = $.types<go_orm>, :$indent="\t") {
    $.go_convert($from-name, $from-type, $to-name, $to-type, :$indent)
  }
  method go_convert_to_pb($from-name = $.names<go_orm>, $from-type = $.types<go_orm>, $to-name = $.names<go_proto>, $to-type = $.types<go_proto>, :$indent="\t") {
    $.go_convert($from-name, $from-type, $to-name, $to-type, :$indent)
  }

  method protobuf_type_from_go_orm {
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

  method push($b) {
    if $.name ne $b.name {
      say "field update: $.name != {$b.name}, ignore";
      return
    }
    # say "update {self} to $b";
    %!names = %( |$b.names, |%.names );
    %!types = %( |$b.types, |%.types );
    %!go-attrs = %( |$b.go-attrs, |%.go-attrs );
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
      when 'json' | 'protobuf' { make $name => $<expr>.Str.split(',').list }
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
  my $ignore-indent;
  my $lines = gather for $filename.IO.lines {
    my $line = $_.subst(/<<LL__(\w+)__TT>>/, {"\{\{ $0 \}\}"}, :g);
    my $line_trim = $line.trim;
    if $ignore {
      # say "'$ignore', '$line_trim'";
      given $ignore {
        when $line_trim { $ignore = Str }
        when $lead_ignore.trim { $ignore = Str; proceed }
        default {
          take $ignore-indent ~ $lead_ignore ~ $line.subst(/^$ignore-indent/)
        }
      }
      next
    }
    if $line_trim.starts-with($lead_ignore) || $line_trim === $lead_ignore.trim {
      $ignore = $line_trim;
      $ignore-indent = $line.match(rx:s/^\s*/).Str;
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
    take $line;
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
        @result.push(Table.new($struct_name, $parse-tag, @fields));
        @fields = [];
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
my $proto = Template::Mustache.render($proto_template, %{
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
(@structs Z, @structs_pb).flat.map: -> $a, $b { $a.push($b) };
my @converter_imports = @structs.map({ $_.go_imports }).flat.unique(:with(&[eq]));
say @converter_imports;
# say @structs;
my $converter_template = load_template($worksapce.add("templates/LL__converter__TT.go"), lead_str=>'// ');
my $converter_code = Template::Mustache.render($converter_template, %{
  go_imports => @converter_imports,
  go_package => 'model',
  :@structs,
});
say $converter_code;
$worksapce.add('dao/model/converter.gen.go').spurt($converter_code);

run <go mod tidy>, cwd=>$worksapce;
