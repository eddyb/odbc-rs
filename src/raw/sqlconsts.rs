/*TODO: use proper types for all consants*/

/* sql.h constants*/

#[allow(non_camel_case_types)]

pub const ODBCVER: u16 = 0x0351;
pub const SQL_NULL_DATA: i16 = (-1);
pub const SQL_DATA_AT_EXEC: i16 = (-2);
pub const SQL_SUCCESS: u16 = 0;
pub const SQL_SUCCESS_WITH_INFO: u16 = 1;
pub const SQL_NO_DATA: u16 = 100;
pub const SQL_ERROR: i16 = (-1);
pub const SQL_INVALID_HANDLE: i16 = (-2);
pub const SQL_STILL_EXECUTING: u16 = 2;
pub const SQL_NEED_DATA: u16 = 99;
pub const SQL_NTS: i16 = (-3);
pub const SQL_NTSL: i32 = (-3);
pub const SQL_MAX_MESSAGE_LENGTH: u16 = 512;
pub const SQL_DATE_LEN: u16 = 10;
pub const SQL_TIME_LEN: u16 = 8;
pub const SQL_TIMESTAMP_LEN: u16 = 19;
pub const SQL_HANDLE_ENV: i16 = 1;
pub const SQL_HANDLE_DBC: i16 = 2;
pub const SQL_HANDLE_STMT: i16 = 3;
pub const SQL_HANDLE_DESC: i16 = 4;
pub const SQL_ATTR_OUTPUT_NTS: u16 = 10001;
pub const SQL_ATTR_AUTO_IPD: u16 = 10001;
pub const SQL_ATTR_METADATA_ID: u16 = 10014;
pub const SQL_ATTR_APP_ROW_DESC: u16 = 10010;
pub const SQL_ATTR_APP_PARAM_DESC: u16 = 10011;
pub const SQL_ATTR_IMP_ROW_DESC: u16 = 10012;
pub const SQL_ATTR_IMP_PARAM_DESC: u16 = 10013;
pub const SQL_ATTR_CURSOR_SCROLLABLE: i16 = (-1);
pub const SQL_ATTR_CURSOR_SENSITIVITY: i16 = (-2);
pub const SQL_NONSCROLLABLE: u16 = 0;
pub const SQL_SCROLLABLE: u16 = 1;
pub const SQL_DESC_COUNT: u16 = 1001;
pub const SQL_DESC_TYPE: u16 = 1002;
pub const SQL_DESC_LENGTH: u16 = 1003;
pub const SQL_DESC_OCTET_LENGTH_PTR: u16 = 1004;
pub const SQL_DESC_PRECISION: u16 = 1005;
pub const SQL_DESC_SCALE: u16 = 1006;
pub const SQL_DESC_DATETIME_INTERVAL_CODE: u16 = 1007;
pub const SQL_DESC_NULLABLE: u16 = 1008;
pub const SQL_DESC_INDICATOR_PTR: u16 = 1009;
pub const SQL_DESC_DATA_PTR: u16 = 1010;
pub const SQL_DESC_NAME: u16 = 1011;
pub const SQL_DESC_UNNAMED: u16 = 1012;
pub const SQL_DESC_OCTET_LENGTH: u16 = 1013;
pub const SQL_DESC_ALLOC_TYPE: u16 = 1099;
pub const SQL_DIAG_RETURNCODE: u16 = 1;
pub const SQL_DIAG_NUMBER: u16 = 2;
pub const SQL_DIAG_ROW_COUNT: u16 = 3;
pub const SQL_DIAG_SQLSTATE: u16 = 4;
pub const SQL_DIAG_NATIVE: u16 = 5;
pub const SQL_DIAG_MESSAGE_TEXT: u16 = 6;
pub const SQL_DIAG_DYNAMIC_FUNCTION: u16 = 7;
pub const SQL_DIAG_CLASS_ORIGIN: u16 = 8;
pub const SQL_DIAG_SUBCLASS_ORIGIN: u16 = 9;
pub const SQL_DIAG_CONNECTION_NAME: u16 = 10;
pub const SQL_DIAG_SERVER_NAME: u16 = 11;
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: u16 = 12;
pub const SQL_DIAG_ALTER_DOMAIN: u16 = 3;
pub const SQL_DIAG_ALTER_TABLE: u16 = 4;
pub const SQL_DIAG_CALL: u16 = 7;
pub const SQL_DIAG_CREATE_ASSERTION: u16 = 6;
pub const SQL_DIAG_CREATE_CHARACTER_SET: u16 = 8;
pub const SQL_DIAG_CREATE_COLLATION: u16 = 10;
pub const SQL_DIAG_CREATE_DOMAIN: u16 = 23;
pub const SQL_DIAG_CREATE_INDEX: i16 = (-1);
pub const SQL_DIAG_CREATE_SCHEMA: u16 = 64;
pub const SQL_DIAG_CREATE_TABLE: u16 = 77;
pub const SQL_DIAG_CREATE_TRANSLATION: u16 = 79;
pub const SQL_DIAG_CREATE_VIEW: u16 = 84;
pub const SQL_DIAG_DELETE_WHERE: u16 = 19;
pub const SQL_DIAG_DROP_ASSERTION: u16 = 24;
pub const SQL_DIAG_DROP_CHARACTER_SET: u16 = 25;
pub const SQL_DIAG_DROP_COLLATION: u16 = 26;
pub const SQL_DIAG_DROP_DOMAIN: u16 = 27;
pub const SQL_DIAG_DROP_INDEX: i16 = (-2);
pub const SQL_DIAG_DROP_SCHEMA: u16 = 31;
pub const SQL_DIAG_DROP_TABLE: u16 = 32;
pub const SQL_DIAG_DROP_TRANSLATION: u16 = 33;
pub const SQL_DIAG_DROP_VIEW: u16 = 36;
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: u16 = 38;
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: u16 = 81;
pub const SQL_DIAG_GRANT: u16 = 48;
pub const SQL_DIAG_INSERT: u16 = 50;
pub const SQL_DIAG_REVOKE: u16 = 59;
pub const SQL_DIAG_SELECT_CURSOR: u16 = 85;
pub const SQL_DIAG_UNKNOWN_STATEMENT: u16 = 0;
pub const SQL_DIAG_UPDATE_WHERE: u16 = 82;
pub const SQL_UNKNOWN_TYPE: u16 = 0;
pub const SQL_CHAR: u16 = 1;
pub const SQL_NUMERIC: u16 = 2;
pub const SQL_DECIMAL: u16 = 3;
pub const SQL_INTEGER: u16 = 4;
pub const SQL_SMALLINT: u16 = 5;
pub const SQL_FLOAT: u16 = 6;
pub const SQL_REAL: u16 = 7;
pub const SQL_DOUBLE: u16 = 8;
pub const SQL_DATETIME: u16 = 9;
pub const SQL_VARCHAR: u16 = 12;
pub const SQL_TYPE_DATE: u16 = 91;
pub const SQL_TYPE_TIME: u16 = 92;
pub const SQL_TYPE_TIMESTAMP: u16 = 93;
pub const SQL_UNSPECIFIED: u16 = 0;
pub const SQL_INSENSITIVE: u16 = 1;
pub const SQL_SENSITIVE: u16 = 2;
pub const SQL_ALL_TYPES: u16 = 0;
pub const SQL_DEFAULT: u16 = 99;
pub const SQL_ARD_TYPE: i16 = (-99);
pub const SQL_CODE_DATE: u16 = 1;
pub const SQL_CODE_TIME: u16 = 2;
pub const SQL_CODE_TIMESTAMP: u16 = 3;
pub const SQL_FALSE: u16 = 0;
pub const SQL_TRUE: u16 = 1;
pub const SQL_NO_NULLS: u16 = 0;
pub const SQL_NULLABLE: u16 = 1;
pub const SQL_NULLABLE_UNKNOWN: u16 = 2;
pub const SQL_PRED_NONE: u16 = 0;
pub const SQL_PRED_CHAR: u16 = 1;
pub const SQL_PRED_BASIC: u16 = 2;
pub const SQL_NAMED: u16 = 0;
pub const SQL_UNNAMED: u16 = 1;
pub const SQL_DESC_ALLOC_AUTO: u16 = 1;
pub const SQL_DESC_ALLOC_USER: u16 = 2;
pub const SQL_CLOSE: u16 = 0;
pub const SQL_DROP: u16 = 1;
pub const SQL_UNBIND: u16 = 2;
pub const SQL_RESET_PARAMS: u16 = 3;
pub const SQL_FETCH_NEXT: u16 = 1;
pub const SQL_FETCH_FIRST: u16 = 2;
pub const SQL_FETCH_LAST: u16 = 3;
pub const SQL_FETCH_PRIOR: u16 = 4;
pub const SQL_FETCH_ABSOLUTE: u16 = 5;
pub const SQL_FETCH_RELATIVE: u16 = 6;
pub const SQL_COMMIT: u16 = 0;
pub const SQL_ROLLBACK: u16 = 1;
pub const SQL_NULL_HENV: u16 = 0;
pub const SQL_NULL_HDBC: u16 = 0;
pub const SQL_NULL_HSTMT: u16 = 0;
pub const SQL_NULL_HDESC: u16 = 0;
pub const SQL_NULL_DESC: u16 = 0;
pub const SQL_NULL_HANDLE: u16 = 0;
pub const SQL_SCOPE_CURROW: u16 = 0;
pub const SQL_SCOPE_TRANSACTION: u16 = 1;
pub const SQL_SCOPE_SESSION: u16 = 2;
pub const SQL_PC_UNKNOWN: u16 = 0;
pub const SQL_PC_NON_PSEUDO: u16 = 1;
pub const SQL_PC_PSEUDO: u16 = 2;
pub const SQL_ROW_IDENTIFIER: u16 = 1;
pub const SQL_INDEX_UNIQUE: u16 = 0;
pub const SQL_INDEX_ALL: u16 = 1;
pub const SQL_INDEX_CLUSTERED: u16 = 1;
pub const SQL_INDEX_HASHED: u16 = 2;
pub const SQL_INDEX_OTHER: u16 = 3;
pub const SQL_API_SQLALLOCCONNECT: u16 = 1;
pub const SQL_API_SQLALLOCENV: u16 = 2;
pub const SQL_API_SQLALLOCHANDLE: u16 = 1001;
pub const SQL_API_SQLALLOCSTMT: u16 = 3;
pub const SQL_API_SQLBINDCOL: u16 = 4;
pub const SQL_API_SQLBINDPARAM: u16 = 1002;
pub const SQL_API_SQLCANCEL: u16 = 5;
pub const SQL_API_SQLCLOSECURSOR: u16 = 1003;
pub const SQL_API_SQLCOLATTRIBUTE: u16 = 6;
pub const SQL_API_SQLCOLUMNS: u16 = 40;
pub const SQL_API_SQLCONNECT: u16 = 7;
pub const SQL_API_SQLCOPYDESC: u16 = 1004;
pub const SQL_API_SQLDATASOURCES: u16 = 57;
pub const SQL_API_SQLDESCRIBECOL: u16 = 8;
pub const SQL_API_SQLDISCONNECT: u16 = 9;
pub const SQL_API_SQLENDTRAN: u16 = 1005;
pub const SQL_API_SQLERROR: u16 = 10;
pub const SQL_API_SQLEXECDIRECT: u16 = 11;
pub const SQL_API_SQLEXECUTE: u16 = 12;
pub const SQL_API_SQLFETCH: u16 = 13;
pub const SQL_API_SQLFETCHSCROLL: u16 = 1021;
pub const SQL_API_SQLFREECONNECT: u16 = 14;
pub const SQL_API_SQLFREEENV: u16 = 15;
pub const SQL_API_SQLFREEHANDLE: u16 = 1006;
pub const SQL_API_SQLFREESTMT: u16 = 16;
pub const SQL_API_SQLGETCONNECTATTR: u16 = 1007;
pub const SQL_API_SQLGETCONNECTOPTION: u16 = 42;
pub const SQL_API_SQLGETCURSORNAME: u16 = 17;
pub const SQL_API_SQLGETDATA: u16 = 43;
pub const SQL_API_SQLGETDESCFIELD: u16 = 1008;
pub const SQL_API_SQLGETDESCREC: u16 = 1009;
pub const SQL_API_SQLGETDIAGFIELD: u16 = 1010;
pub const SQL_API_SQLGETDIAGREC: u16 = 1011;
pub const SQL_API_SQLGETENVATTR: u16 = 1012;
pub const SQL_API_SQLGETFUNCTIONS: u16 = 44;
pub const SQL_API_SQLGETINFO: u16 = 45;
pub const SQL_API_SQLGETSTMTATTR: u16 = 1014;
pub const SQL_API_SQLGETSTMTOPTION: u16 = 46;
pub const SQL_API_SQLGETTYPEINFO: u16 = 47;
pub const SQL_API_SQLNUMRESULTCOLS: u16 = 18;
pub const SQL_API_SQLPARAMDATA: u16 = 48;
pub const SQL_API_SQLPREPARE: u16 = 19;
pub const SQL_API_SQLPUTDATA: u16 = 49;
pub const SQL_API_SQLROWCOUNT: u16 = 20;
pub const SQL_API_SQLSETCONNECTATTR: u16 = 1016;
pub const SQL_API_SQLSETCONNECTOPTION: u16 = 50;
pub const SQL_API_SQLSETCURSORNAME: u16 = 21;
pub const SQL_API_SQLSETDESCFIELD: u16 = 1017;
pub const SQL_API_SQLSETDESCREC: u16 = 1018;
pub const SQL_API_SQLSETENVATTR: u16 = 1019;
pub const SQL_API_SQLSETPARAM: u16 = 22;
pub const SQL_API_SQLSETSTMTATTR: u16 = 1020;
pub const SQL_API_SQLSETSTMTOPTION: u16 = 51;
pub const SQL_API_SQLSPECIALCOLUMNS: u16 = 52;
pub const SQL_API_SQLSTATISTICS: u16 = 53;
pub const SQL_API_SQLTABLES: u16 = 54;
pub const SQL_API_SQLTRANSACT: u16 = 23;
pub const SQL_MAX_DRIVER_CONNECTIONS: u16 = 0;
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: u16 = *&SQL_MAX_DRIVER_CONNECTIONS;
pub const SQL_MAX_CONCURRENT_ACTIVITIES: u16 = 1;
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: u16 = *&SQL_MAX_CONCURRENT_ACTIVITIES;
pub const SQL_DATA_SOURCE_NAME: u16 = 2;
pub const SQL_FETCH_DIRECTION: u16 = 8;
pub const SQL_SERVER_NAME: u16 = 13;
pub const SQL_SEARCH_PATTERN_ESCAPE: u16 = 14;
pub const SQL_DBMS_NAME: u16 = 17;
pub const SQL_DBMS_VER: u16 = 18;
pub const SQL_ACCESSIBLE_TABLES: u16 = 19;
pub const SQL_ACCESSIBLE_PROCEDURES: u16 = 20;
pub const SQL_CURSOR_COMMIT_BEHAVIOR: u16 = 23;
pub const SQL_DATA_SOURCE_READ_ONLY: u16 = 25;
pub const SQL_DEFAULT_TXN_ISOLATION: u16 = 26;
pub const SQL_IDENTIFIER_CASE: u16 = 28;
pub const SQL_IDENTIFIER_QUOTE_CHAR: u16 = 29;
pub const SQL_MAX_COLUMN_NAME_LEN: u16 = 30;
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: u16 = *&SQL_MAX_COLUMN_NAME_LEN;
pub const SQL_MAX_CURSOR_NAME_LEN: u16 = 31;
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: u16 = *&SQL_MAX_CURSOR_NAME_LEN;
pub const SQL_MAX_SCHEMA_NAME_LEN: u16 = 32;
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: u16 = *&SQL_MAX_SCHEMA_NAME_LEN;
pub const SQL_MAX_CATALOG_NAME_LEN: u16 = 34;
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: u16 = *&SQL_MAX_CATALOG_NAME_LEN;
pub const SQL_MAX_TABLE_NAME_LEN: u16 = 35;
pub const SQL_SCROLL_CONCURRENCY: u16 = 43;
pub const SQL_TXN_CAPABLE: u16 = 46;
pub const SQL_TRANSACTION_CAPABLE: u16 = *&SQL_TXN_CAPABLE;
pub const SQL_USER_NAME: u16 = 47;
pub const SQL_TXN_ISOLATION_OPTION: u16 = 72;
pub const SQL_TRANSACTION_ISOLATION_OPTION: u16 = *&SQL_TXN_ISOLATION_OPTION;
pub const SQL_INTEGRITY: u16 = 73;
pub const SQL_GETDATA_EXTENSIONS: u16 = 81;
pub const SQL_NULL_COLLATION: u16 = 85;
pub const SQL_ALTER_TABLE: u16 = 86;
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: u16 = 90;
pub const SQL_SPECIAL_CHARACTERS: u16 = 94;
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: u16 = 97;
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: u16 = *&SQL_MAX_COLUMNS_IN_GROUP_BY;
pub const SQL_MAX_COLUMNS_IN_INDEX: u16 = 98;
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: u16 = *&SQL_MAX_COLUMNS_IN_INDEX;
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: u16 = 99;
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: u16 = *&SQL_MAX_COLUMNS_IN_ORDER_BY;
pub const SQL_MAX_COLUMNS_IN_SELECT: u16 = 100;
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: u16 = *&SQL_MAX_COLUMNS_IN_SELECT;
pub const SQL_MAX_COLUMNS_IN_TABLE: u16 = 101;
pub const SQL_MAX_INDEX_SIZE: u16 = 102;
pub const SQL_MAXIMUM_INDEX_SIZE: u16 = *&SQL_MAX_INDEX_SIZE;
pub const SQL_MAX_ROW_SIZE: u16 = 104;
pub const SQL_MAXIMUM_ROW_SIZE: u16 = *&SQL_MAX_ROW_SIZE;
pub const SQL_MAX_STATEMENT_LEN: u16 = 105;
pub const SQL_MAXIMUM_STATEMENT_LENGTH: u16 = *&SQL_MAX_STATEMENT_LEN;
pub const SQL_MAX_TABLES_IN_SELECT: u16 = 106;
pub const SQL_MAXIMUM_TABLES_IN_SELECT: u16 = *&SQL_MAX_TABLES_IN_SELECT;
pub const SQL_MAX_USER_NAME_LEN: u16 = 107;
pub const SQL_MAXIMUM_USER_NAME_LENGTH: u16 = *&SQL_MAX_USER_NAME_LEN;
pub const SQL_OJ_CAPABILITIES: u16 = 115;
pub const SQL_OUTER_JOIN_CAPABILITIES: u16 = *&SQL_OJ_CAPABILITIES;
pub const SQL_XOPEN_CLI_YEAR: u16 = 10000;
pub const SQL_CURSOR_SENSITIVITY: u16 = 10001;
pub const SQL_DESCRIBE_PARAMETER: u16 = 10002;
pub const SQL_CATALOG_NAME: u16 = 10003;
pub const SQL_COLLATION_SEQ: u16 = 10004;
pub const SQL_MAX_IDENTIFIER_LEN: u16 = 10005;
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: u16 = *&SQL_MAX_IDENTIFIER_LEN;
pub const SQL_AT_ADD_COLUMN: u32 = 0x00000001;
pub const SQL_AT_DROP_COLUMN: u32 = 0x00000002;
pub const SQL_AT_ADD_CONSTRAINT: u32 = 0x00000008;
pub const SQL_AT_COLUMN_SINGLE: u32 = 0x00000020;
pub const SQL_AT_ADD_COLUMN_DEFAULT: u32 = 0x00000040;
pub const SQL_AT_ADD_COLUMN_COLLATION: u32 = 0x00000080;
pub const SQL_AT_SET_COLUMN_DEFAULT: u32 = 0x00000100;
pub const SQL_AT_DROP_COLUMN_DEFAULT: u32 = 0x00000200;
pub const SQL_AT_DROP_COLUMN_CASCADE: u32 = 0x00000400;
pub const SQL_AT_DROP_COLUMN_RESTRICT: u32 = 0x00000800;
pub const SQL_AT_ADD_TABLE_CONSTRAINT: u32 = 0x00001000;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: u16 = 0x00002000;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: u16 = 0x00004000;
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: u16 = 0x00008000;
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: u32 = 0x00010000;
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 0x00020000;
pub const SQL_AT_CONSTRAINT_DEFERRABLE: u32 = 0x00040000;
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: u32 = 0x00080000;
pub const SQL_AM_NONE: u16 = 0;
pub const SQL_AM_CONNECTION: u16 = 1;
pub const SQL_AM_STATEMENT: u16 = 2;
pub const SQL_CB_DELETE: u16 = 0;
pub const SQL_CB_CLOSE: u16 = 1;
pub const SQL_CB_PRESERVE: u16 = 2;
pub const SQL_FD_FETCH_NEXT: u16 = 0x00000001;
pub const SQL_FD_FETCH_FIRST: u16 = 0x00000002;
pub const SQL_FD_FETCH_LAST: u16 = 0x00000004;
pub const SQL_FD_FETCH_PRIOR: u16 = 0x00000008;
pub const SQL_FD_FETCH_ABSOLUTE: u16 = 0x00000010;
pub const SQL_FD_FETCH_RELATIVE: u16 = 0x00000020;
pub const SQL_GD_ANY_COLUMN: u16 = 0x00000001;
pub const SQL_GD_ANY_ORDER: u16 = 0x00000002;
pub const SQL_IC_UPPER: u16 = 1;
pub const SQL_IC_LOWER: u16 = 2;
pub const SQL_IC_SENSITIVE: u16 = 3;
pub const SQL_IC_MIXED: u16 = 4;
pub const SQL_OJ_LEFT: u16 = 0x00000001;
pub const SQL_OJ_RIGHT: u16 = 0x00000002;
pub const SQL_OJ_FULL: u16 = 0x00000004;
pub const SQL_OJ_NESTED: u16 = 0x00000008;
pub const SQL_OJ_NOT_ORDERED: u16 = 0x00000010;
pub const SQL_OJ_INNER: u16 = 0x00000020;
pub const SQL_OJ_ALL_COMPARISON_OPS: u16 = 0x00000040;
pub const SQL_SCCO_READ_ONLY: u16 = 0x00000001;
pub const SQL_SCCO_LOCK: u16 = 0x00000002;
pub const SQL_SCCO_OPT_ROWVER: u16 = 0x00000004;
pub const SQL_SCCO_OPT_VALUES: u16 = 0x00000008;
pub const SQL_TC_NONE: u16 = 0;
pub const SQL_TC_DML: u16 = 1;
pub const SQL_TC_ALL: u16 = 2;
pub const SQL_TC_DDL_COMMIT: u16 = 3;
pub const SQL_TC_DDL_IGNORE: u16 = 4;
pub const SQL_TXN_READ_UNCOMMITTED: u16 = 0x00000001;
pub const SQL_TRANSACTION_READ_UNCOMMITTED: u16 = *&SQL_TXN_READ_UNCOMMITTED;
pub const SQL_TXN_READ_COMMITTED: u16 = 0x00000002;
pub const SQL_TRANSACTION_READ_COMMITTED: u16 = *&SQL_TXN_READ_COMMITTED;
pub const SQL_TXN_REPEATABLE_READ: u16 = 0x00000004;
pub const SQL_TRANSACTION_REPEATABLE_READ: u16 = *&SQL_TXN_REPEATABLE_READ;
pub const SQL_TXN_SERIALIZABLE: u16 = 0x00000008;
pub const SQL_TRANSACTION_SERIALIZABLE: u16 = *&SQL_TXN_SERIALIZABLE;
pub const SQL_NC_HIGH: u16 = 0;
pub const SQL_NC_LOW: u16 = 1;
