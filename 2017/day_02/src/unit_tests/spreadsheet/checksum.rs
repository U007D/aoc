use spreadsheet::{CellType, Spreadsheet};
use super::{given, run,
            Result};

#[derive(Clone, Debug)]
struct Env {
    spreadsheet: Result<Spreadsheet>,
    result: Option<CellType>,
}

impl Env {
    fn new(data: &str) -> Self {
        Self {
            spreadsheet: Spreadsheet::new(data),
            result: None,
        }
    }
}

#[test]
fn tests() {
    run(&given("a spreadsheet with test data set 1", Env::new("5 1 9 5\n7 5 3\n2 4 6 8"), |ctx| {
        ctx.when("the checksum is calculated", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.spreadsheet.clone().unwrap().checksum());
            });
            let expected_result = 18;

            ctx.then("the result should be 18", move |env| {
                assert!(env.result.unwrap() == expected_result);
            });
        });
    }));

    run(&given("a spreadsheet with test data set 2", Env::new("62	1649	1731	76	51	1295	349	719	52	1984	2015	2171	981	1809	181	1715\n161	99	1506	1658	84	78	533	242	1685	86	107	1548	670	960	1641	610\n95	2420	2404	2293	542	2107	2198	121	109	209	2759	1373	1446	905	1837	111\n552	186	751	527	696	164	114	530	558	307	252	200	481	142	205	479\n581	1344	994	1413	120	112	656	1315	1249	193	1411	1280	110	103	74	1007\n2536	5252	159	179	4701	1264	1400	2313	4237	161	142	4336	1061	3987	2268	4669\n3270	1026	381	185	293	3520	1705	1610	3302	628	3420	524	3172	244	295	39\n4142	1835	4137	3821	3730	2094	468	141	150	3982	147	4271	1741	2039	4410	179\n1796	83	2039	1252	84	1641	2165	1218	1936	335	1807	2268	66	102	1977	2445\n96	65	201	275	257	282	233	60	57	200	216	134	72	105	81	212\n3218	5576	5616	5253	178	3317	6147	5973	2424	274	4878	234	200	4781	5372	276\n4171	2436	134	3705	3831	3952	2603	115	660	125	610	152	4517	587	1554	619\n2970	128	2877	1565	1001	167	254	2672	59	473	2086	181	1305	162	1663	2918\n271	348	229	278	981	1785	2290	516	473	2037	737	2291	2521	1494	1121	244\n2208	2236	1451	621	1937	1952	865	61	1934	49	1510	50	1767	59	194	1344\n94	2312	2397	333	1192	106	2713	2351	2650	2663	703	157	89	510	1824	125"), |ctx| {
        ctx.when("the checksum is calculated", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.spreadsheet.clone().unwrap().checksum());
            });
            let expected_result = 44216;

            ctx.then("the result should be 18", move |env| {
                assert!(env.result.unwrap() == expected_result);
            });
        });
    }));
}

