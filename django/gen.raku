my $worksapce = $*PROGRAM.parent;

say "working in $worksapce";

run <git clean -fX src test.sqlite3>, cwd=>$worksapce;
$worksapce.add('src/models_gen.py').spurt: '';
# run <ls -la>, $worksapce.add("src/migrations").absolute;
say "inspectdb";
my $model-py = run(<python manage.py inspectdb --settings=misc.prod --database=data>, cwd=>$worksapce, :out).out.slurp(:close);
$model-py = $model-py
  .subst("managed = False", "managed = True", :g)
  .subst(rx:s/class DieselSchemaMigrations.*\'__diesel_schema_migrations\'\n\n\n/)
  .subst(rx:s/class DjangoMigrations.*\'django_migrations\'\n\n\n/);
$worksapce.add('src/models_gen.py').spurt($model-py);

run <python manage.py makemigrations>, cwd=>$worksapce;
run <python manage.py migrate>, cwd=>$worksapce;
run <python manage.py migrate --settings=misc.prod>, cwd=>$worksapce;
# this line would introduce DjangoMigrations
# run <python manage.py migrate --settings=misc.prod --database=data>, cwd=>$worksapce;

run <protoc -I../micro --python_out=. protos/db.proto>, cwd=>$worksapce;

# my $model-out-py = run(<python manage.py inspectdb>, cwd=>$worksapce, :out).out.slurp(:close);
# $worksapce.add('src/models_out_gen.py').spurt($model-out-py);

("test.default", <python manage.py inspectdb>,
 "prod.default", <python manage.py inspectdb --settings=misc.prod>,
 "prod.data", <python manage.py inspectdb --settings=misc.prod --database=data>).map: -> $prompt, @cmd {
  say "$prompt:";
  run(@cmd, cwd=>$worksapce, :out).out.slurp(:close).lines.grep(/^class/).join("\n").subst("class ", "  ", :g).subst("(models.Model):", :g).say;
}
