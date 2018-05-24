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
fn january2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[0], "2018-01-01", '月', Some("元日".into()));
    assert_koyomi!(c[1], "2018-01-02", '火', None);
    assert_koyomi!(c[2], "2018-01-03", '水', None);
    assert_koyomi!(c[3], "2018-01-04", '木', None);
    assert_koyomi!(c[4], "2018-01-05", '金', None);
    assert_koyomi!(c[5], "2018-01-06", '土', None);
    assert_koyomi!(c[6], "2018-01-07", '日', None);
    assert_koyomi!(c[7], "2018-01-08", '月', Some("成人の日".into()));
    assert_koyomi!(c[8], "2018-01-09", '火', None);
    assert_koyomi!(c[9], "2018-01-10", '水', None);
    assert_koyomi!(c[10], "2018-01-11", '木', None);
    assert_koyomi!(c[11], "2018-01-12", '金', None);
    assert_koyomi!(c[12], "2018-01-13", '土', None);
    assert_koyomi!(c[13], "2018-01-14", '日', None);
    assert_koyomi!(c[14], "2018-01-15", '月', None);
    assert_koyomi!(c[15], "2018-01-16", '火', None);
    assert_koyomi!(c[16], "2018-01-17", '水', None);
    assert_koyomi!(c[17], "2018-01-18", '木', None);
    assert_koyomi!(c[18], "2018-01-19", '金', None);
    assert_koyomi!(c[19], "2018-01-20", '土', None);
    assert_koyomi!(c[20], "2018-01-21", '日', None);
    assert_koyomi!(c[21], "2018-01-22", '月', None);
    assert_koyomi!(c[22], "2018-01-23", '火', None);
    assert_koyomi!(c[23], "2018-01-24", '水', None);
    assert_koyomi!(c[24], "2018-01-25", '木', None);
    assert_koyomi!(c[25], "2018-01-26", '金', None);
    assert_koyomi!(c[26], "2018-01-27", '土', None);
    assert_koyomi!(c[27], "2018-01-28", '日', None);
    assert_koyomi!(c[28], "2018-01-29", '月', None);
    assert_koyomi!(c[29], "2018-01-30", '火', None);
    assert_koyomi!(c[30], "2018-01-31", '水', None);
}

#[test]
fn february2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[31], "2018-02-01", '木', None);
    assert_koyomi!(c[32], "2018-02-02", '金', None);
    assert_koyomi!(c[33], "2018-02-03", '土', None);
    assert_koyomi!(c[34], "2018-02-04", '日', None);
    assert_koyomi!(c[35], "2018-02-05", '月', None);
    assert_koyomi!(c[36], "2018-02-06", '火', None);
    assert_koyomi!(c[37], "2018-02-07", '水', None);
    assert_koyomi!(c[38], "2018-02-08", '木', None);
    assert_koyomi!(c[39], "2018-02-09", '金', None);
    assert_koyomi!(c[40], "2018-02-10", '土', None);
    assert_koyomi!(c[41], "2018-02-11", '日', Some("建国記念日".into()));
    assert_koyomi!(c[42], "2018-02-12", '月', Some("振替休日".into()));
    assert_koyomi!(c[43], "2018-02-13", '火', None);
    assert_koyomi!(c[44], "2018-02-14", '水', None);
    assert_koyomi!(c[45], "2018-02-15", '木', None);
    assert_koyomi!(c[46], "2018-02-16", '金', None);
    assert_koyomi!(c[47], "2018-02-17", '土', None);
    assert_koyomi!(c[48], "2018-02-18", '日', None);
    assert_koyomi!(c[49], "2018-02-19", '月', None);
    assert_koyomi!(c[50], "2018-02-20", '火', None);
    assert_koyomi!(c[51], "2018-02-21", '水', None);
    assert_koyomi!(c[52], "2018-02-22", '木', None);
    assert_koyomi!(c[53], "2018-02-23", '金', None);
    assert_koyomi!(c[54], "2018-02-24", '土', None);
    assert_koyomi!(c[55], "2018-02-25", '日', None);
    assert_koyomi!(c[56], "2018-02-26", '月', None);
    assert_koyomi!(c[57], "2018-02-27", '火', None);
    assert_koyomi!(c[58], "2018-02-28", '水', None);
}

#[test]
fn march2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[59], "2018-03-01", '木', None);
    assert_koyomi!(c[60], "2018-03-02", '金', None);
    assert_koyomi!(c[61], "2018-03-03", '土', None);
    assert_koyomi!(c[62], "2018-03-04", '日', None);
    assert_koyomi!(c[63], "2018-03-05", '月', None);
    assert_koyomi!(c[64], "2018-03-06", '火', None);
    assert_koyomi!(c[65], "2018-03-07", '水', None);
    assert_koyomi!(c[66], "2018-03-08", '木', None);
    assert_koyomi!(c[67], "2018-03-09", '金', None);
    assert_koyomi!(c[68], "2018-03-10", '土', None);
    assert_koyomi!(c[69], "2018-03-11", '日', None);
    assert_koyomi!(c[70], "2018-03-12", '月', None);
    assert_koyomi!(c[71], "2018-03-13", '火', None);
    assert_koyomi!(c[72], "2018-03-14", '水', None);
    assert_koyomi!(c[73], "2018-03-15", '木', None);
    assert_koyomi!(c[74], "2018-03-16", '金', None);
    assert_koyomi!(c[75], "2018-03-17", '土', None);
    assert_koyomi!(c[76], "2018-03-18", '日', None);
    assert_koyomi!(c[77], "2018-03-19", '月', None);
    assert_koyomi!(c[78], "2018-03-20", '火', None);
    assert_koyomi!(c[79], "2018-03-21", '水', Some("春分の日".into()));
    assert_koyomi!(c[80], "2018-03-22", '木', None);
    assert_koyomi!(c[81], "2018-03-23", '金', None);
    assert_koyomi!(c[82], "2018-03-24", '土', None);
    assert_koyomi!(c[83], "2018-03-25", '日', None);
    assert_koyomi!(c[84], "2018-03-26", '月', None);
    assert_koyomi!(c[85], "2018-03-27", '火', None);
    assert_koyomi!(c[86], "2018-03-28", '水', None);
    assert_koyomi!(c[87], "2018-03-29", '木', None);
    assert_koyomi!(c[88], "2018-03-30", '金', None);
    assert_koyomi!(c[89], "2018-03-31", '土', None);
}

#[test]
fn april2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[90], "2018-04-01", '日', None);
    assert_koyomi!(c[91], "2018-04-02", '月', None);
    assert_koyomi!(c[92], "2018-04-03", '火', None);
    assert_koyomi!(c[93], "2018-04-04", '水', None);
    assert_koyomi!(c[94], "2018-04-05", '木', None);
    assert_koyomi!(c[95], "2018-04-06", '金', None);
    assert_koyomi!(c[96], "2018-04-07", '土', None);
    assert_koyomi!(c[97], "2018-04-08", '日', None);
    assert_koyomi!(c[98], "2018-04-09", '月', None);
    assert_koyomi!(c[99], "2018-04-10", '火', None);
    assert_koyomi!(c[100], "2018-04-11", '水', None);
    assert_koyomi!(c[101], "2018-04-12", '木', None);
    assert_koyomi!(c[102], "2018-04-13", '金', None);
    assert_koyomi!(c[103], "2018-04-14", '土', None);
    assert_koyomi!(c[104], "2018-04-15", '日', None);
    assert_koyomi!(c[105], "2018-04-16", '月', None);
    assert_koyomi!(c[106], "2018-04-17", '火', None);
    assert_koyomi!(c[107], "2018-04-18", '水', None);
    assert_koyomi!(c[108], "2018-04-19", '木', None);
    assert_koyomi!(c[109], "2018-04-20", '金', None);
    assert_koyomi!(c[110], "2018-04-21", '土', None);
    assert_koyomi!(c[111], "2018-04-22", '日', None);
    assert_koyomi!(c[112], "2018-04-23", '月', None);
    assert_koyomi!(c[113], "2018-04-24", '火', None);
    assert_koyomi!(c[114], "2018-04-25", '水', None);
    assert_koyomi!(c[115], "2018-04-26", '木', None);
    assert_koyomi!(c[116], "2018-04-27", '金', None);
    assert_koyomi!(c[117], "2018-04-28", '土', None);
    assert_koyomi!(c[118], "2018-04-29", '日', Some("昭和の日".into()));
    assert_koyomi!(c[119], "2018-04-30", '月', Some("振替休日".into()));
}

#[test]
fn may2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[120], "2018-05-01", '火', None);
    assert_koyomi!(c[121], "2018-05-02", '水', None);
    assert_koyomi!(c[122], "2018-05-03", '木', Some("憲法記念日".into()));
    assert_koyomi!(c[123], "2018-05-04", '金', Some("みどりの日".into()));
    assert_koyomi!(c[124], "2018-05-05", '土', Some("こどもの日".into()));
    assert_koyomi!(c[125], "2018-05-06", '日', None);
    assert_koyomi!(c[126], "2018-05-07", '月', None);
    assert_koyomi!(c[127], "2018-05-08", '火', None);
    assert_koyomi!(c[128], "2018-05-09", '水', None);
    assert_koyomi!(c[129], "2018-05-10", '木', None);
    assert_koyomi!(c[130], "2018-05-11", '金', None);
    assert_koyomi!(c[131], "2018-05-12", '土', None);
    assert_koyomi!(c[132], "2018-05-13", '日', None);
    assert_koyomi!(c[133], "2018-05-14", '月', None);
    assert_koyomi!(c[134], "2018-05-15", '火', None);
    assert_koyomi!(c[135], "2018-05-16", '水', None);
    assert_koyomi!(c[136], "2018-05-17", '木', None);
    assert_koyomi!(c[137], "2018-05-18", '金', None);
    assert_koyomi!(c[138], "2018-05-19", '土', None);
    assert_koyomi!(c[139], "2018-05-20", '日', None);
    assert_koyomi!(c[140], "2018-05-21", '月', None);
    assert_koyomi!(c[141], "2018-05-22", '火', None);
    assert_koyomi!(c[142], "2018-05-23", '水', None);
    assert_koyomi!(c[143], "2018-05-24", '木', None);
    assert_koyomi!(c[144], "2018-05-25", '金', None);
    assert_koyomi!(c[145], "2018-05-26", '土', None);
    assert_koyomi!(c[146], "2018-05-27", '日', None);
    assert_koyomi!(c[147], "2018-05-28", '月', None);
    assert_koyomi!(c[148], "2018-05-29", '火', None);
    assert_koyomi!(c[149], "2018-05-30", '水', None);
    assert_koyomi!(c[150], "2018-05-31", '木', None);
}

#[test]
fn june2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[151], "2018-06-01", '金', None);
    assert_koyomi!(c[152], "2018-06-02", '土', None);
    assert_koyomi!(c[153], "2018-06-03", '日', None);
    assert_koyomi!(c[154], "2018-06-04", '月', None);
    assert_koyomi!(c[155], "2018-06-05", '火', None);
    assert_koyomi!(c[156], "2018-06-06", '水', None);
    assert_koyomi!(c[157], "2018-06-07", '木', None);
    assert_koyomi!(c[158], "2018-06-08", '金', None);
    assert_koyomi!(c[159], "2018-06-09", '土', None);
    assert_koyomi!(c[160], "2018-06-10", '日', None);
    assert_koyomi!(c[161], "2018-06-11", '月', None);
    assert_koyomi!(c[162], "2018-06-12", '火', None);
    assert_koyomi!(c[163], "2018-06-13", '水', None);
    assert_koyomi!(c[164], "2018-06-14", '木', None);
    assert_koyomi!(c[165], "2018-06-15", '金', None);
    assert_koyomi!(c[166], "2018-06-16", '土', None);
    assert_koyomi!(c[167], "2018-06-17", '日', None);
    assert_koyomi!(c[168], "2018-06-18", '月', None);
    assert_koyomi!(c[169], "2018-06-19", '火', None);
    assert_koyomi!(c[170], "2018-06-20", '水', None);
    assert_koyomi!(c[171], "2018-06-21", '木', None);
    assert_koyomi!(c[172], "2018-06-22", '金', None);
    assert_koyomi!(c[173], "2018-06-23", '土', None);
    assert_koyomi!(c[174], "2018-06-24", '日', None);
    assert_koyomi!(c[175], "2018-06-25", '月', None);
    assert_koyomi!(c[176], "2018-06-26", '火', None);
    assert_koyomi!(c[177], "2018-06-27", '水', None);
    assert_koyomi!(c[178], "2018-06-28", '木', None);
    assert_koyomi!(c[179], "2018-06-29", '金', None);
    assert_koyomi!(c[180], "2018-06-30", '土', None);
}

#[test]
fn july2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[181], "2018-07-01", '日', None);
    assert_koyomi!(c[182], "2018-07-02", '月', None);
    assert_koyomi!(c[183], "2018-07-03", '火', None);
    assert_koyomi!(c[184], "2018-07-04", '水', None);
    assert_koyomi!(c[185], "2018-07-05", '木', None);
    assert_koyomi!(c[186], "2018-07-06", '金', None);
    assert_koyomi!(c[187], "2018-07-07", '土', None);
    assert_koyomi!(c[188], "2018-07-08", '日', None);
    assert_koyomi!(c[189], "2018-07-09", '月', None);
    assert_koyomi!(c[190], "2018-07-10", '火', None);
    assert_koyomi!(c[191], "2018-07-11", '水', None);
    assert_koyomi!(c[192], "2018-07-12", '木', None);
    assert_koyomi!(c[193], "2018-07-13", '金', None);
    assert_koyomi!(c[194], "2018-07-14", '土', None);
    assert_koyomi!(c[195], "2018-07-15", '日', None);
    assert_koyomi!(c[196], "2018-07-16", '月', Some("海の日".into()));
    assert_koyomi!(c[197], "2018-07-17", '火', None);
    assert_koyomi!(c[198], "2018-07-18", '水', None);
    assert_koyomi!(c[199], "2018-07-19", '木', None);
    assert_koyomi!(c[200], "2018-07-20", '金', None);
    assert_koyomi!(c[201], "2018-07-21", '土', None);
    assert_koyomi!(c[202], "2018-07-22", '日', None);
    assert_koyomi!(c[203], "2018-07-23", '月', None);
    assert_koyomi!(c[204], "2018-07-24", '火', None);
    assert_koyomi!(c[205], "2018-07-25", '水', None);
    assert_koyomi!(c[206], "2018-07-26", '木', None);
    assert_koyomi!(c[207], "2018-07-27", '金', None);
    assert_koyomi!(c[208], "2018-07-28", '土', None);
    assert_koyomi!(c[209], "2018-07-29", '日', None);
    assert_koyomi!(c[210], "2018-07-30", '月', None);
    assert_koyomi!(c[211], "2018-07-31", '火', None);
}

#[test]
fn august2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[212], "2018-08-01", '水', None);
    assert_koyomi!(c[213], "2018-08-02", '木', None);
    assert_koyomi!(c[214], "2018-08-03", '金', None);
    assert_koyomi!(c[215], "2018-08-04", '土', None);
    assert_koyomi!(c[216], "2018-08-05", '日', None);
    assert_koyomi!(c[217], "2018-08-06", '月', None);
    assert_koyomi!(c[218], "2018-08-07", '火', None);
    assert_koyomi!(c[219], "2018-08-08", '水', None);
    assert_koyomi!(c[220], "2018-08-09", '木', None);
    assert_koyomi!(c[221], "2018-08-10", '金', None);
    assert_koyomi!(c[222], "2018-08-11", '土', Some("山の日".into()));
    assert_koyomi!(c[223], "2018-08-12", '日', None);
    assert_koyomi!(c[224], "2018-08-13", '月', None);
    assert_koyomi!(c[225], "2018-08-14", '火', None);
    assert_koyomi!(c[226], "2018-08-15", '水', None);
    assert_koyomi!(c[227], "2018-08-16", '木', None);
    assert_koyomi!(c[228], "2018-08-17", '金', None);
    assert_koyomi!(c[229], "2018-08-18", '土', None);
    assert_koyomi!(c[230], "2018-08-19", '日', None);
    assert_koyomi!(c[231], "2018-08-20", '月', None);
    assert_koyomi!(c[232], "2018-08-21", '火', None);
    assert_koyomi!(c[233], "2018-08-22", '水', None);
    assert_koyomi!(c[234], "2018-08-23", '木', None);
    assert_koyomi!(c[235], "2018-08-24", '金', None);
    assert_koyomi!(c[236], "2018-08-25", '土', None);
    assert_koyomi!(c[237], "2018-08-26", '日', None);
    assert_koyomi!(c[238], "2018-08-27", '月', None);
    assert_koyomi!(c[239], "2018-08-28", '火', None);
    assert_koyomi!(c[240], "2018-08-29", '水', None);
    assert_koyomi!(c[241], "2018-08-30", '木', None);
    assert_koyomi!(c[242], "2018-08-31", '金', None);
}

#[test]
fn september2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[243], "2018-09-01", '土', None);
    assert_koyomi!(c[244], "2018-09-02", '日', None);
    assert_koyomi!(c[245], "2018-09-03", '月', None);
    assert_koyomi!(c[246], "2018-09-04", '火', None);
    assert_koyomi!(c[247], "2018-09-05", '水', None);
    assert_koyomi!(c[248], "2018-09-06", '木', None);
    assert_koyomi!(c[249], "2018-09-07", '金', None);
    assert_koyomi!(c[250], "2018-09-08", '土', None);
    assert_koyomi!(c[251], "2018-09-09", '日', None);
    assert_koyomi!(c[252], "2018-09-10", '月', None);
    assert_koyomi!(c[253], "2018-09-11", '火', None);
    assert_koyomi!(c[254], "2018-09-12", '水', None);
    assert_koyomi!(c[255], "2018-09-13", '木', None);
    assert_koyomi!(c[256], "2018-09-14", '金', None);
    assert_koyomi!(c[257], "2018-09-15", '土', None);
    assert_koyomi!(c[258], "2018-09-16", '日', None);
    assert_koyomi!(c[259], "2018-09-17", '月', Some("敬老の日".into()));
    assert_koyomi!(c[260], "2018-09-18", '火', None);
    assert_koyomi!(c[261], "2018-09-19", '水', None);
    assert_koyomi!(c[262], "2018-09-20", '木', None);
    assert_koyomi!(c[263], "2018-09-21", '金', None);
    assert_koyomi!(c[264], "2018-09-22", '土', None);
    assert_koyomi!(c[265], "2018-09-23", '日', Some("秋分の日".into()));
    assert_koyomi!(c[266], "2018-09-24", '月', Some("振替休日".into()));
    assert_koyomi!(c[267], "2018-09-25", '火', None);
    assert_koyomi!(c[268], "2018-09-26", '水', None);
    assert_koyomi!(c[269], "2018-09-27", '木', None);
    assert_koyomi!(c[270], "2018-09-28", '金', None);
    assert_koyomi!(c[271], "2018-09-29", '土', None);
    assert_koyomi!(c[272], "2018-09-30", '日', None);
}

#[test]
fn october2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[273], "2018-10-01", '月', None);
    assert_koyomi!(c[274], "2018-10-02", '火', None);
    assert_koyomi!(c[275], "2018-10-03", '水', None);
    assert_koyomi!(c[276], "2018-10-04", '木', None);
    assert_koyomi!(c[277], "2018-10-05", '金', None);
    assert_koyomi!(c[278], "2018-10-06", '土', None);
    assert_koyomi!(c[279], "2018-10-07", '日', None);
    assert_koyomi!(c[280], "2018-10-08", '月', Some("体育の日".into()));
    assert_koyomi!(c[281], "2018-10-09", '火', None);
    assert_koyomi!(c[282], "2018-10-10", '水', None);
    assert_koyomi!(c[283], "2018-10-11", '木', None);
    assert_koyomi!(c[284], "2018-10-12", '金', None);
    assert_koyomi!(c[285], "2018-10-13", '土', None);
    assert_koyomi!(c[286], "2018-10-14", '日', None);
    assert_koyomi!(c[287], "2018-10-15", '月', None);
    assert_koyomi!(c[288], "2018-10-16", '火', None);
    assert_koyomi!(c[289], "2018-10-17", '水', None);
    assert_koyomi!(c[290], "2018-10-18", '木', None);
    assert_koyomi!(c[291], "2018-10-19", '金', None);
    assert_koyomi!(c[292], "2018-10-20", '土', None);
    assert_koyomi!(c[293], "2018-10-21", '日', None);
    assert_koyomi!(c[294], "2018-10-22", '月', None);
    assert_koyomi!(c[295], "2018-10-23", '火', None);
    assert_koyomi!(c[296], "2018-10-24", '水', None);
    assert_koyomi!(c[297], "2018-10-25", '木', None);
    assert_koyomi!(c[298], "2018-10-26", '金', None);
    assert_koyomi!(c[299], "2018-10-27", '土', None);
    assert_koyomi!(c[300], "2018-10-28", '日', None);
    assert_koyomi!(c[301], "2018-10-29", '月', None);
    assert_koyomi!(c[302], "2018-10-30", '火', None);
    assert_koyomi!(c[303], "2018-10-31", '水', None);
}

#[test]
fn november2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[304], "2018-11-01", '木', None);
    assert_koyomi!(c[305], "2018-11-02", '金', None);
    assert_koyomi!(c[306], "2018-11-03", '土', Some("文化の日".into()));
    assert_koyomi!(c[307], "2018-11-04", '日', None);
    assert_koyomi!(c[308], "2018-11-05", '月', None);
    assert_koyomi!(c[309], "2018-11-06", '火', None);
    assert_koyomi!(c[310], "2018-11-07", '水', None);
    assert_koyomi!(c[311], "2018-11-08", '木', None);
    assert_koyomi!(c[312], "2018-11-09", '金', None);
    assert_koyomi!(c[313], "2018-11-10", '土', None);
    assert_koyomi!(c[314], "2018-11-11", '日', None);
    assert_koyomi!(c[315], "2018-11-12", '月', None);
    assert_koyomi!(c[316], "2018-11-13", '火', None);
    assert_koyomi!(c[317], "2018-11-14", '水', None);
    assert_koyomi!(c[318], "2018-11-15", '木', None);
    assert_koyomi!(c[319], "2018-11-16", '金', None);
    assert_koyomi!(c[320], "2018-11-17", '土', None);
    assert_koyomi!(c[321], "2018-11-18", '日', None);
    assert_koyomi!(c[322], "2018-11-19", '月', None);
    assert_koyomi!(c[323], "2018-11-20", '火', None);
    assert_koyomi!(c[324], "2018-11-21", '水', None);
    assert_koyomi!(c[325], "2018-11-22", '木', None);
    assert_koyomi!(
        c[326],
        "2018-11-23",
        '金',
        Some("勤労感謝の日".into())
    );
    assert_koyomi!(c[327], "2018-11-24", '土', None);
    assert_koyomi!(c[328], "2018-11-25", '日', None);
    assert_koyomi!(c[329], "2018-11-26", '月', None);
    assert_koyomi!(c[330], "2018-11-27", '火', None);
    assert_koyomi!(c[331], "2018-11-28", '水', None);
    assert_koyomi!(c[332], "2018-11-29", '木', None);
    assert_koyomi!(c[333], "2018-11-30", '金', None);
}

#[test]
fn december2018() {
    let c = year_of_calendar(2018);

    assert_koyomi!(c[334], "2018-12-01", '土', None);
    assert_koyomi!(c[335], "2018-12-02", '日', None);
    assert_koyomi!(c[336], "2018-12-03", '月', None);
    assert_koyomi!(c[337], "2018-12-04", '火', None);
    assert_koyomi!(c[338], "2018-12-05", '水', None);
    assert_koyomi!(c[339], "2018-12-06", '木', None);
    assert_koyomi!(c[340], "2018-12-07", '金', None);
    assert_koyomi!(c[341], "2018-12-08", '土', None);
    assert_koyomi!(c[342], "2018-12-09", '日', None);
    assert_koyomi!(c[343], "2018-12-10", '月', None);
    assert_koyomi!(c[344], "2018-12-11", '火', None);
    assert_koyomi!(c[345], "2018-12-12", '水', None);
    assert_koyomi!(c[346], "2018-12-13", '木', None);
    assert_koyomi!(c[347], "2018-12-14", '金', None);
    assert_koyomi!(c[348], "2018-12-15", '土', None);
    assert_koyomi!(c[349], "2018-12-16", '日', None);
    assert_koyomi!(c[350], "2018-12-17", '月', None);
    assert_koyomi!(c[351], "2018-12-18", '火', None);
    assert_koyomi!(c[352], "2018-12-19", '水', None);
    assert_koyomi!(c[353], "2018-12-20", '木', None);
    assert_koyomi!(c[354], "2018-12-21", '金', None);
    assert_koyomi!(c[355], "2018-12-22", '土', None);
    assert_koyomi!(c[356], "2018-12-23", '日', Some("天皇誕生日".into()));
    assert_koyomi!(c[357], "2018-12-24", '月', Some("振替休日".into()));
    assert_koyomi!(c[358], "2018-12-25", '火', None);
    assert_koyomi!(c[359], "2018-12-26", '水', None);
    assert_koyomi!(c[360], "2018-12-27", '木', None);
    assert_koyomi!(c[361], "2018-12-28", '金', None);
    assert_koyomi!(c[362], "2018-12-29", '土', None);
    assert_koyomi!(c[363], "2018-12-30", '日', None);
    assert_koyomi!(c[364], "2018-12-31", '月', None);
}
