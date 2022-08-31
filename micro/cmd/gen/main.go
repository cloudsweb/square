package main

import (
	"strings"

	"gorm.io/driver/postgres"
	"gorm.io/gen"
	"gorm.io/gorm"
)

// generate code
func main() {
	// specify the output directory (default: "./query")
	// ### if you want to query without context constrain, set mode gen.WithoutContext ###
	g := gen.NewGenerator(gen.Config{
		OutPath: "dao/query",
		/* Mode: gen.WithoutContext|gen.WithDefaultQuery*/
		//if you want the nullable field generation property to be pointer type, set FieldNullable true
		FieldNullable: true,
		//if you want to assign field which has default value in `Create` API, set FieldCoverable true, reference: https://gorm.io/docs/create.html#Default-Values
		/* FieldCoverable: true,*/
		// if you want generate field with unsigned integer type, set FieldSignable true
		/* FieldSignable: true,*/
		//if you want to generate index tags from database, set FieldWithIndexTag true
		FieldWithIndexTag: true,
		//if you want to generate type tags from database, set FieldWithTypeTag true
		FieldWithTypeTag: true,
		//if you need unit tests for query code, set WithUnitTest true
		WithUnitTest: true,
	})

	// reuse the database connection in Project or create a connection here
	// if you want to use GenerateModel/GenerateModelAs, UseDB is necessary or it will panic
	db, _ := gorm.Open(postgres.Open("postgresql://localhost:7039/posts?sslmode=disable"))
	tableList, _ := db.Migrator().GetTables()
	g.UseDB(db)

	tables := make([]interface{}, 0)
	for _, table := range tableList {
		if strings.HasPrefix(table, "__") {
			continue
		}
		schema := g.GenerateModelAs(table, db.Config.NamingStrategy.SchemaName(table)+"ORM", gen.FieldRename("alias", "ALIAS"))
		tables = append(tables, schema)
	}
	g.ApplyBasic(tables...)

	g.Execute()
}
