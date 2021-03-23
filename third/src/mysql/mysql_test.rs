use mysql::*;
use mysql::prelude::*;

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[test]
pub fn mysql_simple() {
    let url = "mysql://root:123456@localhost:3306/test";
    let pool = Pool::new(url).unwrap();
    let mut conn = match pool.get_conn() {
        Ok(c) => c,
        Err(e) => panic!("connection error:{:?}", e)
    };

    let mut result = conn.query_iter("select name from t_test").unwrap();
    let mut sets = 0;
    while let Some(result_set) = result.next_set() {
        let result_set = result_set.unwrap();
        sets += 1;

        println!("Result set columns: {:?}", result_set.columns());
        println!(
            "Result set meta: {}, {:?}, {} {}",
            result_set.affected_rows(),
            result_set.last_insert_id(),
            result_set.warnings(),
            result_set.info_str(),
        );
    }
}