IF OBJECT_ID('__diesel_schema_migrations') IS NULL 
CREATE TABLE __diesel_schema_migrations (
    [version] VARCHAR (50) NOT NULL,
    [run_on]  DATETIME2    CONSTRAINT [DEFAULT_NewTable_run_on] DEFAULT GETDATE() NULL,
    CONSTRAINT [PK_NewTable] PRIMARY KEY CLUSTERED ([version] ASC)
);