mod primitives;
use super::backend::*;
use diesel::sql_types::HasSqlType;
use diesel::sql_types::*;
use tiberius::ColumnType;

// pub enum MssqlTypeMetadata {
//     Int,
//     Text,
//     DateTime,
// }
impl HasSqlType<BigInt> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Int8
    }
}

impl HasSqlType<Binary> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::BigBinary
    }
}

impl HasSqlType<Bool> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Bit
    }
}

impl HasSqlType<Date> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Datetime2
    }
}

// impl HasSqlType<Decimal> for Mssql {
//     fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
//         MssqlTypeMetadata::Int
//     }
// }
impl HasSqlType<Double> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Float4
    }
}

impl HasSqlType<Float> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Float4
    }
}

impl HasSqlType<Integer> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Int4
    }
}

// impl HasSqlType<Interval> for Mssql {
//     fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
//         MssqlTypeMetadata::Int
//     }
// }
// impl HasSqlType<Json> for Mssql {
//     fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
//         MssqlTypeMetadata::Int
//     }
// }
impl HasSqlType<SmallInt> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Int1
    }
}

impl HasSqlType<Text> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::NVarchar
    }
}

impl HasSqlType<Time> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Datetime2
    }
}
impl HasSqlType<Timestamp> for Mssql {
    fn metadata(_: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ColumnType::Timen
    }
}
