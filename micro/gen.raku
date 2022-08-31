use Template::Mustache;

my $worksapce = $*PROGRAM.parent;

say "working in $worksapce";

# run <ls -la>, $worksapce.add("dao/model").absolute;
say "go run cmd/gen/main.go";
run <git clean -fxd dao>, cwd=>$worksapce;
# run <go mod tidy>, cwd=>$worksapce;
run <go run cmd/gen/main.go>, cwd=>$worksapce;

my %type_map =
  "time.Time" => "google.protobuf.Timestamp",
  ;
grammar field_grammar {
  rule TOP { <.ws> <ident> <type> "`"<tags>"`" }
  rule tags { <subtag> +  }
  token subtag { <ident>':"'<expr>'"' }
  token expr { <-["]>+ }
  token ident { <[a..zA..Z]>\w* }
  token type { "*"?<[a..zA..Z]><[.\w]>* }
}
class Field {
  has Str $.goname;
  has Str $.gotype;
  has Int $.idx;
  has %.gotags;

  method rawtype {
    my $gotype = $!gotype.subst(/^\*/);
    if %type_map{$gotype} {
      %type_map{$gotype}
    } else {
      $gotype
    }
  }
  method pos-idx {
    $!idx + 1
  }
  method optional {
    $!gotype.starts-with('*')
  }
  method optional-str {
    if $.optional {
      "optional"
    } else {
      "required"
    }
  }
  method name {
    if %!gotags<json> {
      %!gotags<json>.first
    } else {
      $.goname
    }
  }

  method TOP ($/) { make self.new(goname=>$<ident>.Str, gotype=>$<type>.Str, gotags=>$<tags>.made) }
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

sub load_template($filename, Str:D :$lead_add='//+ ', Str:D :$lead_sub='//~ ') {
  my @subs;
  my $lines = gather for $filename.IO.lines {
    my $line = $_;
    my $line_trim = $line.trim;
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

sub load_model($filename) {
  my @file_lines = $filename.IO.lines;
  # TODO: check if only one model and matches filename
  my $struct_name = @file_lines.grep(/^type /).first.match(rx:s/type <(\w+)> struct/).Str;
  my @field_lines = @file_lines.grep(/'`'gorm:/);
  my @fields = @field_lines.map({field_grammar.parse($_, actions=>Field).made});
  @fields = @fields.kv.map(-> $i, $v { $v.clone(idx=>$i) });
  { struct_name => name_st($struct_name), :@fields }
}

sub name_st(Str:D $name) {
  $name.subst(/ORM$/)
}

# get files in dao/model
# ["dao/model/users.gen.go".IO "dao/model/secrets.gen.go".IO "dao/model/posts.gen.go".IO]
my @model_files = $worksapce.add("dao/model").dir: test => { .ends-with(".gen.go") };
# {posts => "dao/model/posts.gen.go".IO, ...}
my %models = @model_files.map({ .basename.subst(/'.'gen'.'go$/) }) Z=> @model_files;
say "load models {%models.keys}";

my @structs = %models.values.map({ load_model($_) }).sort({ $_<struct_name> });
# say $struct_name, @fields;
my $template = load_template($worksapce.add("protos/LL__template__TT.proto"));
# say $template;
my $proto = Template::Mustache.render($template, {
  go_package => './dao/model',
  proto_package => 'square.db',
  :@structs,
});

my $out_proto = "protos/db.proto";
$worksapce.add($out_proto).spurt($proto);
say "run protoc $out_proto";
run ("protoc", "--go_out=.", $worksapce.add($out_proto).Str), cwd=>$worksapce;
run <go mod tidy>, cwd=>$worksapce;
