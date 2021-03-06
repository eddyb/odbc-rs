pub mod raw;

#[test]
fn it_works() {

    use raw::*;
    use std::ffi::{CStr, CString};

    unsafe {
        let mut env : SQLHENV = std::ptr::null_mut();
        SQLAllocEnv(&mut env);

        let mut ret : SQLRETURN;

        let mut name = [0; 1024];
        let mut name_ret : SQLSMALLINT  = 0;

        let mut desc = [0; 1024];
        let mut desc_ret : SQLSMALLINT  = 0;

        println!("Driver list:");
        while {ret = SQLDrivers(env, SQL_FETCH_NEXT, name.as_mut_ptr(), name.len() as i16, &mut name_ret, desc.as_mut_ptr(), desc.len() as i16, &mut desc_ret); ret} & !1 == 0 {
            println!("{:?} - {:?}", CStr::from_ptr(name.as_ptr() as *const i8), CStr::from_ptr(desc.as_ptr() as *const i8));
        }

        println!("DataSource list:");
        while {ret = SQLDataSources(env, SQL_FETCH_NEXT, name.as_mut_ptr(), name.len() as i16, &mut name_ret, desc.as_mut_ptr(), desc.len() as i16, &mut desc_ret); ret} & !1 == 0 {
            println!("{:?} - {:?}", CStr::from_ptr(name.as_ptr() as *const i8), CStr::from_ptr(desc.as_ptr() as *const i8));
        }

        let mut dbc : SQLHDBC = std::ptr::null_mut();
        SQLAllocConnect(env, &mut dbc);

        let dsn = CString::new("DSN=pglocal;Database=crm;Uid=postgres;Password=postgres").unwrap();

        println!("CONNECTING TO {:?}", dsn);

        let dsn_ptr = dsn.into_raw();

        ret = SQLDriverConnect(dbc, std::ptr::null_mut(), dsn_ptr as *mut u8, SQL_NTS, name.as_mut_ptr(), name.len() as i16, &mut name_ret, SQL_DRIVER_NOPROMPT);

        CString::from_raw(dsn_ptr);

        if ret & !1 == 0 {
            println!("CONNECTED: {:?}", CStr::from_ptr(name.as_ptr() as *const i8));

            let mut stmt : SQLHSTMT = std::ptr::null_mut();
            SQLAllocStmt(dbc, &mut stmt);

            let sql = CString::new("select * from security.user").unwrap();

            println!("EXECUTING SQL {:?}", sql);

            let sql_ptr = sql.into_raw();
            ret = SQLExecDirect(stmt, sql_ptr as *mut u8, SQL_NTSL);
            CString::from_raw(sql_ptr);

            if ret & !1 == 0 {
                let mut columns : SQLSMALLINT = 0;
                SQLNumResultCols(stmt, &mut columns);

                println!("SUCCESSFUL:");

                let mut i = 1;
                while {ret = SQLFetch(stmt); ret} & !1 == 0 {
                    println!("\tROW: {}", i);

                    for j in 1..columns {
                        let mut indicator : SQLLEN = 0;
                        let mut buf = [0; 512];
                        ret = SQLGetData(stmt, j as u16, 1, buf.as_mut_ptr() as SQLPOINTER, buf.len() as i64, &mut indicator);
                        if ret & !1 == 0 {
                            if indicator == -1 {
                                println!("Column {}: NULL", j);
                            } else {
                                println!("Column {}: {:?}", j, CStr::from_ptr(buf.as_ptr() as *const i8));
                            }
                        }
                    }

                    i += 1;
                }
            } else {
                println!("FAILED:");
                let mut i = 1;
                let mut native : SQLINTEGER  = 0;
                while {ret = SQLGetDiagRec(SQL_HANDLE_STMT, stmt, i, name.as_mut_ptr(), &mut native, desc.as_mut_ptr(), desc.len() as i16, &mut desc_ret); ret} & !1 == 0 {
                    println!("\t{:?}:{}:{}:{:?}", CStr::from_ptr(name.as_ptr() as *const i8), i, native,  CStr::from_ptr(desc.as_ptr() as *const i8));
                    i += 1;
                }
            }

            SQLFreeHandle(SQL_HANDLE_STMT, stmt);
            SQLDisconnect(dbc);
        } else {
            println!("NOT CONNECTED: {:?}", CStr::from_ptr(name.as_ptr() as *const i8));
            let mut i = 1;
            let mut native : SQLINTEGER  = 0;
            while {ret = SQLGetDiagRec(SQL_HANDLE_DBC, dbc, i, name.as_mut_ptr(), &mut native, desc.as_mut_ptr(), desc.len() as i16, &mut desc_ret); ret} & !1 == 0 {
                println!("\t{:?}:{}:{}:{:?}", CStr::from_ptr(name.as_ptr() as *const i8), i, native,  CStr::from_ptr(desc.as_ptr() as *const i8));
                i += 1;
            }
        }

        SQLFreeConnect(dbc);
        SQLFreeEnv(env);
    }

    println!("BYE!");

}
