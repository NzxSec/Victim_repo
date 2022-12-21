fn zeller(mut year: u32, mut month: u32, day: u32)
{
    if month <= 2
    {
        year -= 1;
        month += 12;
    }
    else
    { 
        let w: u32 = year + year / 4 - year / 100 + year / 400 + (13 * month + 8) / 5 + day;
        let h: usize = (w % 7) as usize;
        let ws = vec!["日曜日", "月曜日", "火曜日", "水曜日", "木曜日", "金曜日", "土曜日"];
        
        println!("{}", ws[h]);
    } 
}

fn main()
{
    zeller(2022, 12, 21);
}
