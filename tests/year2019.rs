extern crate koyomi;

macro_rules! assert_koyomi {
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr) => {
        assert_eq!($e1.to_string(), $e2);
        assert_eq!($e1.weekday().japanese(), $e3);
        assert_eq!($e1.holiday(), $e4);
    };
}

fn year_of_calendar(year: usize) -> Vec<koyomi::Date> {
    let year = format!("{}", year);
    let calendar = koyomi::Calendar::build().single(&year).finalize().unwrap();
    calendar.make()
}

#[test]
fn january2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[0], "2019-01-01", '火', Some("元日".into()));
    assert_koyomi!(c[1], "2019-01-02", '水', None);
    assert_koyomi!(c[2], "2019-01-03", '木', None);
    assert_koyomi!(c[3], "2019-01-04", '金', None);
    assert_koyomi!(c[4], "2019-01-05", '土', None);
    assert_koyomi!(c[5], "2019-01-06", '日', None);
    assert_koyomi!(c[6], "2019-01-07", '月', None);
    assert_koyomi!(c[7], "2019-01-08", '火', None);
    assert_koyomi!(c[8], "2019-01-09", '水', None);
    assert_koyomi!(c[9], "2019-01-10", '木', None);
    assert_koyomi!(c[10], "2019-01-11", '金', None);
    assert_koyomi!(c[11], "2019-01-12", '土', None);
    assert_koyomi!(c[12], "2019-01-13", '日', None);
    assert_koyomi!(c[13], "2019-01-14", '月', Some("成人の日".into()));
    assert_koyomi!(c[14], "2019-01-15", '火', None);
    assert_koyomi!(c[15], "2019-01-16", '水', None);
    assert_koyomi!(c[16], "2019-01-17", '木', None);
    assert_koyomi!(c[17], "2019-01-18", '金', None);
    assert_koyomi!(c[18], "2019-01-19", '土', None);
    assert_koyomi!(c[19], "2019-01-20", '日', None);
    assert_koyomi!(c[20], "2019-01-21", '月', None);
    assert_koyomi!(c[21], "2019-01-22", '火', None);
    assert_koyomi!(c[22], "2019-01-23", '水', None);
    assert_koyomi!(c[23], "2019-01-24", '木', None);
    assert_koyomi!(c[24], "2019-01-25", '金', None);
    assert_koyomi!(c[25], "2019-01-26", '土', None);
    assert_koyomi!(c[26], "2019-01-27", '日', None);
    assert_koyomi!(c[27], "2019-01-28", '月', None);
    assert_koyomi!(c[28], "2019-01-29", '火', None);
    assert_koyomi!(c[29], "2019-01-30", '水', None);
    assert_koyomi!(c[30], "2019-01-31", '木', None);
}

#[test]
fn february2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[31], "2019-02-01", '金', None);
    assert_koyomi!(c[32], "2019-02-02", '土', None);
    assert_koyomi!(c[33], "2019-02-03", '日', None);
    assert_koyomi!(c[34], "2019-02-04", '月', None);
    assert_koyomi!(c[35], "2019-02-05", '火', None);
    assert_koyomi!(c[36], "2019-02-06", '水', None);
    assert_koyomi!(c[37], "2019-02-07", '木', None);
    assert_koyomi!(c[38], "2019-02-08", '金', None);
    assert_koyomi!(c[39], "2019-02-09", '土', None);
    assert_koyomi!(c[40], "2019-02-10", '日', None);
    assert_koyomi!(c[41], "2019-02-11", '月', Some("建国記念日".into()));
    assert_koyomi!(c[42], "2019-02-12", '火', None);
    assert_koyomi!(c[43], "2019-02-13", '水', None);
    assert_koyomi!(c[44], "2019-02-14", '木', None);
    assert_koyomi!(c[45], "2019-02-15", '金', None);
    assert_koyomi!(c[46], "2019-02-16", '土', None);
    assert_koyomi!(c[47], "2019-02-17", '日', None);
    assert_koyomi!(c[48], "2019-02-18", '月', None);
    assert_koyomi!(c[49], "2019-02-19", '火', None);
    assert_koyomi!(c[50], "2019-02-20", '水', None);
    assert_koyomi!(c[51], "2019-02-21", '木', None);
    assert_koyomi!(c[52], "2019-02-22", '金', None);
    assert_koyomi!(c[53], "2019-02-23", '土', None);
    assert_koyomi!(c[54], "2019-02-24", '日', None);
    assert_koyomi!(c[55], "2019-02-25", '月', None);
    assert_koyomi!(c[56], "2019-02-26", '火', None);
    assert_koyomi!(c[57], "2019-02-27", '水', None);
    assert_koyomi!(c[58], "2019-02-28", '木', None);
}

#[test]
fn march2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[59], "2019-03-01", '金', None);
    assert_koyomi!(c[60], "2019-03-02", '土', None);
    assert_koyomi!(c[61], "2019-03-03", '日', None);
    assert_koyomi!(c[62], "2019-03-04", '月', None);
    assert_koyomi!(c[63], "2019-03-05", '火', None);
    assert_koyomi!(c[64], "2019-03-06", '水', None);
    assert_koyomi!(c[65], "2019-03-07", '木', None);
    assert_koyomi!(c[66], "2019-03-08", '金', None);
    assert_koyomi!(c[67], "2019-03-09", '土', None);
    assert_koyomi!(c[68], "2019-03-10", '日', None);
    assert_koyomi!(c[69], "2019-03-11", '月', None);
    assert_koyomi!(c[70], "2019-03-12", '火', None);
    assert_koyomi!(c[71], "2019-03-13", '水', None);
    assert_koyomi!(c[72], "2019-03-14", '木', None);
    assert_koyomi!(c[73], "2019-03-15", '金', None);
    assert_koyomi!(c[74], "2019-03-16", '土', None);
    assert_koyomi!(c[75], "2019-03-17", '日', None);
    assert_koyomi!(c[76], "2019-03-18", '月', None);
    assert_koyomi!(c[77], "2019-03-19", '火', None);
    assert_koyomi!(c[78], "2019-03-20", '水', None);
    assert_koyomi!(c[79], "2019-03-21", '木', Some("春分の日".into()));
    assert_koyomi!(c[80], "2019-03-22", '金', None);
    assert_koyomi!(c[81], "2019-03-23", '土', None);
    assert_koyomi!(c[82], "2019-03-24", '日', None);
    assert_koyomi!(c[83], "2019-03-25", '月', None);
    assert_koyomi!(c[84], "2019-03-26", '火', None);
    assert_koyomi!(c[85], "2019-03-27", '水', None);
    assert_koyomi!(c[86], "2019-03-28", '木', None);
    assert_koyomi!(c[87], "2019-03-29", '金', None);
    assert_koyomi!(c[88], "2019-03-30", '土', None);
    assert_koyomi!(c[89], "2019-03-31", '日', None);
}

#[test]
fn april2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[90], "2019-04-01", '月', None);
    assert_koyomi!(c[91], "2019-04-02", '火', None);
    assert_koyomi!(c[92], "2019-04-03", '水', None);
    assert_koyomi!(c[93], "2019-04-04", '木', None);
    assert_koyomi!(c[94], "2019-04-05", '金', None);
    assert_koyomi!(c[95], "2019-04-06", '土', None);
    assert_koyomi!(c[96], "2019-04-07", '日', None);
    assert_koyomi!(c[97], "2019-04-08", '月', None);
    assert_koyomi!(c[98], "2019-04-09", '火', None);
    assert_koyomi!(c[99], "2019-04-10", '水', None);
    assert_koyomi!(c[100], "2019-04-11", '木', None);
    assert_koyomi!(c[101], "2019-04-12", '金', None);
    assert_koyomi!(c[102], "2019-04-13", '土', None);
    assert_koyomi!(c[103], "2019-04-14", '日', None);
    assert_koyomi!(c[104], "2019-04-15", '月', None);
    assert_koyomi!(c[105], "2019-04-16", '火', None);
    assert_koyomi!(c[106], "2019-04-17", '水', None);
    assert_koyomi!(c[107], "2019-04-18", '木', None);
    assert_koyomi!(c[108], "2019-04-19", '金', None);
    assert_koyomi!(c[109], "2019-04-20", '土', None);
    assert_koyomi!(c[110], "2019-04-21", '日', None);
    assert_koyomi!(c[111], "2019-04-22", '月', None);
    assert_koyomi!(c[112], "2019-04-23", '火', None);
    assert_koyomi!(c[113], "2019-04-24", '水', None);
    assert_koyomi!(c[114], "2019-04-25", '木', None);
    assert_koyomi!(c[115], "2019-04-26", '金', None);
    assert_koyomi!(c[116], "2019-04-27", '土', None);
    assert_koyomi!(c[117], "2019-04-28", '日', None);
    assert_koyomi!(c[118], "2019-04-29", '月', Some("昭和の日".into()));
    assert_koyomi!(c[119], "2019-04-30", '火', Some("国民の休日".into()));
}

#[test]
fn may2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[120], "2019-05-01", '水', Some("新天皇即位日".into()));
    assert_koyomi!(c[121], "2019-05-02", '木', Some("国民の休日".into()));
    assert_koyomi!(c[122], "2019-05-03", '金', Some("憲法記念日".into()));
    assert_koyomi!(c[123], "2019-05-04", '土', Some("みどりの日".into()));
    assert_koyomi!(c[124], "2019-05-05", '日', Some("こどもの日".into()));
    assert_koyomi!(c[125], "2019-05-06", '月', Some("振替休日".into()));
    assert_koyomi!(c[126], "2019-05-07", '火', None);
    assert_koyomi!(c[127], "2019-05-08", '水', None);
    assert_koyomi!(c[128], "2019-05-09", '木', None);
    assert_koyomi!(c[129], "2019-05-10", '金', None);
    assert_koyomi!(c[130], "2019-05-11", '土', None);
    assert_koyomi!(c[131], "2019-05-12", '日', None);
    assert_koyomi!(c[132], "2019-05-13", '月', None);
    assert_koyomi!(c[133], "2019-05-14", '火', None);
    assert_koyomi!(c[134], "2019-05-15", '水', None);
    assert_koyomi!(c[135], "2019-05-16", '木', None);
    assert_koyomi!(c[136], "2019-05-17", '金', None);
    assert_koyomi!(c[137], "2019-05-18", '土', None);
    assert_koyomi!(c[138], "2019-05-19", '日', None);
    assert_koyomi!(c[139], "2019-05-20", '月', None);
    assert_koyomi!(c[140], "2019-05-21", '火', None);
    assert_koyomi!(c[141], "2019-05-22", '水', None);
    assert_koyomi!(c[142], "2019-05-23", '木', None);
    assert_koyomi!(c[143], "2019-05-24", '金', None);
    assert_koyomi!(c[144], "2019-05-25", '土', None);
    assert_koyomi!(c[145], "2019-05-26", '日', None);
    assert_koyomi!(c[146], "2019-05-27", '月', None);
    assert_koyomi!(c[147], "2019-05-28", '火', None);
    assert_koyomi!(c[148], "2019-05-29", '水', None);
    assert_koyomi!(c[149], "2019-05-30", '木', None);
    assert_koyomi!(c[150], "2019-05-31", '金', None);
}

#[test]
fn june2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[151], "2019-06-01", '土', None);
    assert_koyomi!(c[152], "2019-06-02", '日', None);
    assert_koyomi!(c[153], "2019-06-03", '月', None);
    assert_koyomi!(c[154], "2019-06-04", '火', None);
    assert_koyomi!(c[155], "2019-06-05", '水', None);
    assert_koyomi!(c[156], "2019-06-06", '木', None);
    assert_koyomi!(c[157], "2019-06-07", '金', None);
    assert_koyomi!(c[158], "2019-06-08", '土', None);
    assert_koyomi!(c[159], "2019-06-09", '日', None);
    assert_koyomi!(c[160], "2019-06-10", '月', None);
    assert_koyomi!(c[161], "2019-06-11", '火', None);
    assert_koyomi!(c[162], "2019-06-12", '水', None);
    assert_koyomi!(c[163], "2019-06-13", '木', None);
    assert_koyomi!(c[164], "2019-06-14", '金', None);
    assert_koyomi!(c[165], "2019-06-15", '土', None);
    assert_koyomi!(c[166], "2019-06-16", '日', None);
    assert_koyomi!(c[167], "2019-06-17", '月', None);
    assert_koyomi!(c[168], "2019-06-18", '火', None);
    assert_koyomi!(c[169], "2019-06-19", '水', None);
    assert_koyomi!(c[170], "2019-06-20", '木', None);
    assert_koyomi!(c[171], "2019-06-21", '金', None);
    assert_koyomi!(c[172], "2019-06-22", '土', None);
    assert_koyomi!(c[173], "2019-06-23", '日', None);
    assert_koyomi!(c[174], "2019-06-24", '月', None);
    assert_koyomi!(c[175], "2019-06-25", '火', None);
    assert_koyomi!(c[176], "2019-06-26", '水', None);
    assert_koyomi!(c[177], "2019-06-27", '木', None);
    assert_koyomi!(c[178], "2019-06-28", '金', None);
    assert_koyomi!(c[179], "2019-06-29", '土', None);
    assert_koyomi!(c[180], "2019-06-30", '日', None);
}

#[test]
fn july2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[181], "2019-07-01", '月', None);
    assert_koyomi!(c[182], "2019-07-02", '火', None);
    assert_koyomi!(c[183], "2019-07-03", '水', None);
    assert_koyomi!(c[184], "2019-07-04", '木', None);
    assert_koyomi!(c[185], "2019-07-05", '金', None);
    assert_koyomi!(c[186], "2019-07-06", '土', None);
    assert_koyomi!(c[187], "2019-07-07", '日', None);
    assert_koyomi!(c[188], "2019-07-08", '月', None);
    assert_koyomi!(c[189], "2019-07-09", '火', None);
    assert_koyomi!(c[190], "2019-07-10", '水', None);
    assert_koyomi!(c[191], "2019-07-11", '木', None);
    assert_koyomi!(c[192], "2019-07-12", '金', None);
    assert_koyomi!(c[193], "2019-07-13", '土', None);
    assert_koyomi!(c[194], "2019-07-14", '日', None);
    assert_koyomi!(c[195], "2019-07-15", '月', Some("海の日".into()));
    assert_koyomi!(c[196], "2019-07-16", '火', None);
    assert_koyomi!(c[197], "2019-07-17", '水', None);
    assert_koyomi!(c[198], "2019-07-18", '木', None);
    assert_koyomi!(c[199], "2019-07-19", '金', None);
    assert_koyomi!(c[200], "2019-07-20", '土', None);
    assert_koyomi!(c[201], "2019-07-21", '日', None);
    assert_koyomi!(c[202], "2019-07-22", '月', None);
    assert_koyomi!(c[203], "2019-07-23", '火', None);
    assert_koyomi!(c[204], "2019-07-24", '水', None);
    assert_koyomi!(c[205], "2019-07-25", '木', None);
    assert_koyomi!(c[206], "2019-07-26", '金', None);
    assert_koyomi!(c[207], "2019-07-27", '土', None);
    assert_koyomi!(c[208], "2019-07-28", '日', None);
    assert_koyomi!(c[209], "2019-07-29", '月', None);
    assert_koyomi!(c[210], "2019-07-30", '火', None);
    assert_koyomi!(c[211], "2019-07-31", '水', None);
}

#[test]
fn august2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[212], "2019-08-01", '木', None);
    assert_koyomi!(c[213], "2019-08-02", '金', None);
    assert_koyomi!(c[214], "2019-08-03", '土', None);
    assert_koyomi!(c[215], "2019-08-04", '日', None);
    assert_koyomi!(c[216], "2019-08-05", '月', None);
    assert_koyomi!(c[217], "2019-08-06", '火', None);
    assert_koyomi!(c[218], "2019-08-07", '水', None);
    assert_koyomi!(c[219], "2019-08-08", '木', None);
    assert_koyomi!(c[220], "2019-08-09", '金', None);
    assert_koyomi!(c[221], "2019-08-10", '土', None);
    assert_koyomi!(c[222], "2019-08-11", '日', Some("山の日".into()));
    assert_koyomi!(c[223], "2019-08-12", '月', Some("振替休日".into()));
    assert_koyomi!(c[224], "2019-08-13", '火', None);
    assert_koyomi!(c[225], "2019-08-14", '水', None);
    assert_koyomi!(c[226], "2019-08-15", '木', None);
    assert_koyomi!(c[227], "2019-08-16", '金', None);
    assert_koyomi!(c[228], "2019-08-17", '土', None);
    assert_koyomi!(c[229], "2019-08-18", '日', None);
    assert_koyomi!(c[230], "2019-08-19", '月', None);
    assert_koyomi!(c[231], "2019-08-20", '火', None);
    assert_koyomi!(c[232], "2019-08-21", '水', None);
    assert_koyomi!(c[233], "2019-08-22", '木', None);
    assert_koyomi!(c[234], "2019-08-23", '金', None);
    assert_koyomi!(c[235], "2019-08-24", '土', None);
    assert_koyomi!(c[236], "2019-08-25", '日', None);
    assert_koyomi!(c[237], "2019-08-26", '月', None);
    assert_koyomi!(c[238], "2019-08-27", '火', None);
    assert_koyomi!(c[239], "2019-08-28", '水', None);
    assert_koyomi!(c[240], "2019-08-29", '木', None);
    assert_koyomi!(c[241], "2019-08-30", '金', None);
    assert_koyomi!(c[242], "2019-08-31", '土', None);
}

#[test]
fn september2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[243], "2019-09-01", '日', None);
    assert_koyomi!(c[244], "2019-09-02", '月', None);
    assert_koyomi!(c[245], "2019-09-03", '火', None);
    assert_koyomi!(c[246], "2019-09-04", '水', None);
    assert_koyomi!(c[247], "2019-09-05", '木', None);
    assert_koyomi!(c[248], "2019-09-06", '金', None);
    assert_koyomi!(c[249], "2019-09-07", '土', None);
    assert_koyomi!(c[250], "2019-09-08", '日', None);
    assert_koyomi!(c[251], "2019-09-09", '月', None);
    assert_koyomi!(c[252], "2019-09-10", '火', None);
    assert_koyomi!(c[253], "2019-09-11", '水', None);
    assert_koyomi!(c[254], "2019-09-12", '木', None);
    assert_koyomi!(c[255], "2019-09-13", '金', None);
    assert_koyomi!(c[256], "2019-09-14", '土', None);
    assert_koyomi!(c[257], "2019-09-15", '日', None);
    assert_koyomi!(c[258], "2019-09-16", '月', Some("敬老の日".into()));
    assert_koyomi!(c[259], "2019-09-17", '火', None);
    assert_koyomi!(c[260], "2019-09-18", '水', None);
    assert_koyomi!(c[261], "2019-09-19", '木', None);
    assert_koyomi!(c[262], "2019-09-20", '金', None);
    assert_koyomi!(c[263], "2019-09-21", '土', None);
    assert_koyomi!(c[264], "2019-09-22", '日', None);
    assert_koyomi!(c[265], "2019-09-23", '月', Some("秋分の日".into()));
    assert_koyomi!(c[266], "2019-09-24", '火', None);
    assert_koyomi!(c[267], "2019-09-25", '水', None);
    assert_koyomi!(c[268], "2019-09-26", '木', None);
    assert_koyomi!(c[269], "2019-09-27", '金', None);
    assert_koyomi!(c[270], "2019-09-28", '土', None);
    assert_koyomi!(c[271], "2019-09-29", '日', None);
    assert_koyomi!(c[272], "2019-09-30", '月', None);
}

#[test]
fn october2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[273], "2019-10-01", '火', None);
    assert_koyomi!(c[274], "2019-10-02", '水', None);
    assert_koyomi!(c[275], "2019-10-03", '木', None);
    assert_koyomi!(c[276], "2019-10-04", '金', None);
    assert_koyomi!(c[277], "2019-10-05", '土', None);
    assert_koyomi!(c[278], "2019-10-06", '日', None);
    assert_koyomi!(c[279], "2019-10-07", '月', None);
    assert_koyomi!(c[280], "2019-10-08", '火', None);
    assert_koyomi!(c[281], "2019-10-09", '水', None);
    assert_koyomi!(c[282], "2019-10-10", '木', None);
    assert_koyomi!(c[283], "2019-10-11", '金', None);
    assert_koyomi!(c[284], "2019-10-12", '土', None);
    assert_koyomi!(c[285], "2019-10-13", '日', None);
    assert_koyomi!(c[286], "2019-10-14", '月', Some("体育の日".into()));
    assert_koyomi!(c[287], "2019-10-15", '火', None);
    assert_koyomi!(c[288], "2019-10-16", '水', None);
    assert_koyomi!(c[289], "2019-10-17", '木', None);
    assert_koyomi!(c[290], "2019-10-18", '金', None);
    assert_koyomi!(c[291], "2019-10-19", '土', None);
    assert_koyomi!(c[292], "2019-10-20", '日', None);
    assert_koyomi!(c[293], "2019-10-21", '月', None);
    assert_koyomi!(c[294], "2019-10-22", '火', Some("即位礼正殿の儀".into()));
    assert_koyomi!(c[295], "2019-10-23", '水', None);
    assert_koyomi!(c[296], "2019-10-24", '木', None);
    assert_koyomi!(c[297], "2019-10-25", '金', None);
    assert_koyomi!(c[298], "2019-10-26", '土', None);
    assert_koyomi!(c[299], "2019-10-27", '日', None);
    assert_koyomi!(c[300], "2019-10-28", '月', None);
    assert_koyomi!(c[301], "2019-10-29", '火', None);
    assert_koyomi!(c[302], "2019-10-30", '水', None);
    assert_koyomi!(c[303], "2019-10-31", '木', None);
}

#[test]
fn november2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[304], "2019-11-01", '金', None);
    assert_koyomi!(c[305], "2019-11-02", '土', None);
    assert_koyomi!(c[306], "2019-11-03", '日', Some("文化の日".into()));
    assert_koyomi!(c[307], "2019-11-04", '月', Some("振替休日".into()));
    assert_koyomi!(c[308], "2019-11-05", '火', None);
    assert_koyomi!(c[309], "2019-11-06", '水', None);
    assert_koyomi!(c[310], "2019-11-07", '木', None);
    assert_koyomi!(c[311], "2019-11-08", '金', None);
    assert_koyomi!(c[312], "2019-11-09", '土', None);
    assert_koyomi!(c[313], "2019-11-10", '日', None);
    assert_koyomi!(c[314], "2019-11-11", '月', None);
    assert_koyomi!(c[315], "2019-11-12", '火', None);
    assert_koyomi!(c[316], "2019-11-13", '水', None);
    assert_koyomi!(c[317], "2019-11-14", '木', None);
    assert_koyomi!(c[318], "2019-11-15", '金', None);
    assert_koyomi!(c[319], "2019-11-16", '土', None);
    assert_koyomi!(c[320], "2019-11-17", '日', None);
    assert_koyomi!(c[321], "2019-11-18", '月', None);
    assert_koyomi!(c[322], "2019-11-19", '火', None);
    assert_koyomi!(c[323], "2019-11-20", '水', None);
    assert_koyomi!(c[324], "2019-11-21", '木', None);
    assert_koyomi!(c[325], "2019-11-22", '金', None);
    assert_koyomi!(c[326], "2019-11-23", '土', Some("勤労感謝の日".into()));
    assert_koyomi!(c[327], "2019-11-24", '日', None);
    assert_koyomi!(c[328], "2019-11-25", '月', None);
    assert_koyomi!(c[329], "2019-11-26", '火', None);
    assert_koyomi!(c[330], "2019-11-27", '水', None);
    assert_koyomi!(c[331], "2019-11-28", '木', None);
    assert_koyomi!(c[332], "2019-11-29", '金', None);
    assert_koyomi!(c[333], "2019-11-30", '土', None);
}

#[test]
fn december2019() {
    let c = year_of_calendar(2019);

    assert_koyomi!(c[334], "2019-12-01", '日', None);
    assert_koyomi!(c[335], "2019-12-02", '月', None);
    assert_koyomi!(c[336], "2019-12-03", '火', None);
    assert_koyomi!(c[337], "2019-12-04", '水', None);
    assert_koyomi!(c[338], "2019-12-05", '木', None);
    assert_koyomi!(c[339], "2019-12-06", '金', None);
    assert_koyomi!(c[340], "2019-12-07", '土', None);
    assert_koyomi!(c[341], "2019-12-08", '日', None);
    assert_koyomi!(c[342], "2019-12-09", '月', None);
    assert_koyomi!(c[343], "2019-12-10", '火', None);
    assert_koyomi!(c[344], "2019-12-11", '水', None);
    assert_koyomi!(c[345], "2019-12-12", '木', None);
    assert_koyomi!(c[346], "2019-12-13", '金', None);
    assert_koyomi!(c[347], "2019-12-14", '土', None);
    assert_koyomi!(c[348], "2019-12-15", '日', None);
    assert_koyomi!(c[349], "2019-12-16", '月', None);
    assert_koyomi!(c[350], "2019-12-17", '火', None);
    assert_koyomi!(c[351], "2019-12-18", '水', None);
    assert_koyomi!(c[352], "2019-12-19", '木', None);
    assert_koyomi!(c[353], "2019-12-20", '金', None);
    assert_koyomi!(c[354], "2019-12-21", '土', None);
    assert_koyomi!(c[355], "2019-12-22", '日', None);
    assert_koyomi!(c[356], "2019-12-23", '月', None);
    assert_koyomi!(c[357], "2019-12-24", '火', None);
    assert_koyomi!(c[358], "2019-12-25", '水', None);
    assert_koyomi!(c[359], "2019-12-26", '木', None);
    assert_koyomi!(c[360], "2019-12-27", '金', None);
    assert_koyomi!(c[361], "2019-12-28", '土', None);
    assert_koyomi!(c[362], "2019-12-29", '日', None);
    assert_koyomi!(c[363], "2019-12-30", '月', None);
    assert_koyomi!(c[364], "2019-12-31", '火', None);
}
