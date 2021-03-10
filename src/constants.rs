/// 列舉中國十天干：甲、乙、丙、丁、戊、己、更、辛、壬、葵。
pub(crate) const THE_HEAVENLY_STEMS: [&str; 10] =
    ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

/// 列舉中國十天干：甲、乙、丙、丁、戊、己、更、辛、壬、葵。
pub(crate) const THE_HEAVENLY_STEMS_CHARS: [char; 10] =
    ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];

/// 列舉中國十二地支：子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥。
pub(crate) const THE_EARTHLY_BRANCHES: [&str; 12] =
    ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 列舉中國十二地支：子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥。
pub(crate) const THE_EARTHLY_BRANCHES_CHARS: [char; 12] =
    ['子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥'];

/// 列舉中國十二生肖：鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬。
pub(crate) const THE_ZODIAC_SIGNS: [[&str; 2]; 12] = [
    ["鼠", "鼠"],
    ["牛", "牛"],
    ["虎", "虎"],
    ["兔", "兔"],
    ["龍", "龙"],
    ["蛇", "蛇"],
    ["馬", "马"],
    ["羊", "羊"],
    ["猴", "猴"],
    ["雞", "鸡"],
    ["狗", "狗"],
    ["豬", "猪"],
];

/// 列舉中國十二生肖：鼠、牛、虎、兔、龍、蛇、馬、羊、猴、雞、狗、豬。
pub(crate) const THE_ZODIAC_SIGNS_CHARS: [[char; 2]; 12] = [
    ['鼠', '鼠'],
    ['牛', '牛'],
    ['虎', '虎'],
    ['兔', '兔'],
    ['龍', '龙'],
    ['蛇', '蛇'],
    ['馬', '马'],
    ['羊', '羊'],
    ['猴', '猴'],
    ['雞', '鸡'],
    ['狗', '狗'],
    ['豬', '猪'],
];

/// 列舉農曆六十個年份名稱：甲子、乙丑、丙寅、...、癸亥。
pub(crate) const THE_LUNAR_YEARS: [&str; 60] = [
    "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", // 0 / 12
    "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", // 10
    "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", // 8
    "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯", // 6
    "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑", // 4
    "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥", // 2
];

/// 列舉西曆十二個月份名稱：一月、二月、三月、四月、五月、六月、七月、八月、九月、十月、十一月、十二月。
pub(crate) const THE_SOLAR_MONTHS: [&str; 12] = [
    "一月",
    "二月",
    "三月",
    "四月",
    "五月",
    "六月",
    "七月",
    "八月",
    "九月",
    "十月",
    "十一月",
    "十二月",
];

/// 列舉農曆十二個月份名稱：正月、二月、三月、四月、五月、六月、七月、八月、九月、十月、冬月、臘月。閏月緊接其後。
pub(crate) const THE_LUNAR_MONTHS: [[&str; 2]; 31] = [
    ["正月", "正月"],
    ["二月", "二月"],
    ["三月", "三月"],
    ["四月", "四月"],
    ["五月", "五月"],
    ["六月", "六月"],
    ["七月", "七月"],
    ["八月", "八月"],
    ["九月", "九月"],
    ["十月", "十月"],
    ["冬月", "冬月"],
    ["臘月", "腊月"],
    ["閏正月", "闰正月"],
    ["閏二月", "闰二月"],
    ["閏三月", "闰三月"],
    ["閏四月", "闰四月"],
    ["閏五月", "闰五月"],
    ["閏六月", "闰六月"],
    ["閏七月", "闰七月"],
    ["閏八月", "闰八月"],
    ["閏九月", "闰九月"],
    ["閏十月", "闰十月"],
    ["閏冬月", "闰冬月"],
    ["閏臘月", "闰腊月"],
    ["一月", "一月"],             // 補充
    ["十一月", "十一月"],       // 補充
    ["十二月", "十二月"],       // 補充
    ["闰一月", "閏一月"],       // 補充
    ["闰十一月", "閏十一月"], // 補充
    ["闰十二月", "閏十二月"], // 補充
    ["闰臘月", "閏腊月"],       // 排列組合補充
];

/// 列舉西曆三十一個天數名稱：一、二、...、十一、十二、...、二十一、二十二、...、三十、三十一。
pub(crate) const THE_SOLAR_DAYS: [&str; 31] = [
    "一",
    "二",
    "三",
    "四",
    "五",
    "六",
    "七",
    "八",
    "九",
    "十", // 10
    "十一",
    "十二",
    "十三",
    "十四",
    "十五",
    "十六",
    "十七",
    "十八",
    "十九",
    "二十", // 20
    "二十一",
    "二十二",
    "二十三",
    "二十四",
    "二十五",
    "二十六",
    "二十七",
    "二十八",
    "二十九",
    "三十", // 30
    "三十一",
];

/// 列舉農曆三十個天數名稱：初一、初二、...、十一、十二、...、廿一、廿二、...、三十。
pub(crate) const THE_LUNAR_DAYS: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", // 10
    "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", // 20
    "廿一", "廿二", "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十", // 30
];

/// 列舉西曆年用到的數字：零、一、二、...、九。
pub(crate) const THE_SOLAR_YEAR_NUMBERS: [&str; 10] =
    ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// 列舉西曆年用到的數字(根據GB/T)：〇、一、二、...、九。
pub(crate) const THE_SOLAR_YEAR_NUMBERS_2: [&str; 10] =
    ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// 列舉西曆年用到的數字：零、一、二、...、九。
pub(crate) const THE_SOLAR_YEAR_NUMBERS_CHARS: [char; 10] =
    ['零', '一', '二', '三', '四', '五', '六', '七', '八', '九'];

/// u16型別有16位元(15...0)，將第15~3個位元用來分別表示1~13月(含閏年)是否為大月(1為大月有30天；0為小月有29天)。此處有西元1901~2100年的資料。
pub(crate) const BIG_MONTHS: [u16; 200] = [
    0x4AE0, 0xA570, 0x5268, 0xD260, 0xD950, 0x6AA8, 0x56A0, 0x9AD0, 0x4AE8, 0x4AE0, // 1910
    0xA4D8, 0xA4D0, 0xD250, 0xD528, 0xB540, 0xD6A0, 0x96D0, 0x95B0, 0x49B8, 0x4970, // 1920
    0xA4B0, 0xB258, 0x6A50, 0x6D40, 0xADA8, 0x2B60, 0x9570, 0x4978, 0x4970, 0x64B0, // 1930
    0xD4A0, 0xEA50, 0x6D48, 0x5AD0, 0x2B60, 0x9370, 0x92E0, 0xC968, 0xC950, 0xD4A0, // 1940
    0xDA50, 0xB550, 0x56A0, 0xAAD8, 0x25D0, 0x92D0, 0xC958, 0xA950, 0xB4A8, 0x6CA0, // 1950
    0xB550, 0x55A8, 0x4DA0, 0xA5B0, 0x52B8, 0x52B0, 0xA950, 0xE950, 0x6AA0, 0xAD50, // 1960
    0xAB50, 0x4B60, 0xA570, 0xA570, 0x5260, 0xE930, 0xD950, 0x5AA8, 0x56A0, 0x96D0, // 1970
    0x4AE8, 0x4AD0, 0xA4D0, 0xD268, 0xD250, 0xD528, 0xB540, 0xB6A0, 0x96D0, 0x95B0, // 1980
    0x49B0, 0xA4B8, 0xA4B0, 0xB258, 0x6A50, 0x6D40, 0xADA0, 0xAB60, 0x9570, 0x4978, // 1990
    0x4970, 0x64B0, 0x6A50, 0xEA50, 0x6B28, 0x5AC0, 0xAB60, 0x9368, 0x92E0, 0xC960, // 2000
    0xD4A8, 0xD4A0, 0xDA50, 0x5AA8, 0x56A0, 0xAAD8, 0x25D0, 0x92D0, 0xC958, 0xA950, // 2010
    0xB4A0, 0xB550, 0xAD50, 0x55A8, 0x4BA0, 0xA5B0, 0x52B8, 0x52B0, 0xA930, 0x74A8, // 2020
    0x6AA0, 0xAD50, 0x4DA8, 0x4B60, 0xA570, 0xA4E0, 0xD260, 0xE930, 0xD530, 0x5AA0, // 2030
    0x6B50, 0x96D0, 0x4AE8, 0x4AD0, 0xA4D0, 0xD258, 0xD250, 0xD520, 0xDAA0, 0xB5A0, // 2040
    0x56D0, 0x4AD8, 0x49B0, 0xA4B8, 0xA4B0, 0xAA50, 0xB528, 0x6D20, 0xADA0, 0x55B0, // 2050
    0x9370, 0x4978, 0x4970, 0x64B0, 0x6A50, 0xEA50, 0x6B20, 0xAB60, 0xAAE0, 0x92E0, // 2060
    0xC970, 0xC960, 0xD4A8, 0xD4A0, 0xDA50, 0x5AA8, 0x56A0, 0xA6D0, 0x52E8, 0x52D0, // 2070
    0xA958, 0xA950, 0xB4A0, 0xB550, 0xAD50, 0x55A0, 0xA5D0, 0xA5B0, 0x52B0, 0xA938, // 2080
    0x6930, 0x7298, 0x6AA0, 0xAD50, 0x4DA8, 0x4B60, 0xA570, 0x5270, 0xD260, 0xE930, // 2090
    0xD520, 0xDAA0, 0x6B50, 0x56D0, 0x4AE0, 0xA4E8, 0xA4D0, 0xD150, 0xD928, 0xD520, // 2100
];

/// u8型別有8位元(7...0)，將第7~4個位元用來表示第一年是閏幾月，第3~0個位元用來表示第二年是閏幾月。若該年數值為0，表示該年沒有閏月。此處有西元1901~2100年的資料。
pub(crate) const LEAP_MONTHS: [u8; 100] = [
    0x00, 0x50, 0x04, 0x00, 0x20, // 1910
    0x60, 0x05, 0x00, 0x20, 0x70, // 1920
    0x05, 0x00, 0x40, 0x02, 0x06, // 1930
    0x00, 0x50, 0x03, 0x07, 0x00, // 1940
    0x60, 0x04, 0x00, 0x20, 0x70, // 1950
    0x05, 0x00, 0x30, 0x80, 0x06, // 1960
    0x00, 0x40, 0x03, 0x07, 0x00, // 1970
    0x50, 0x04, 0x08, 0x00, 0x60, // 1980
    0x04, 0x0A, 0x00, 0x60, 0x05, // 1990
    0x00, 0x30, 0x80, 0x05, 0x00, // 2000
    0x40, 0x02, 0x07, 0x00, 0x50, // 2010
    0x04, 0x09, 0x00, 0x60, 0x04, // 2020
    0x00, 0x20, 0x60, 0x05, 0x00, // 2030
    0x30, 0xB0, 0x06, 0x00, 0x50, // 2040
    0x02, 0x07, 0x00, 0x50, 0x03, // 2050
    0x08, 0x00, 0x60, 0x04, 0x00, // 2060
    0x30, 0x70, 0x05, 0x00, 0x40, // 2070
    0x80, 0x06, 0x00, 0x40, 0x03, // 2080
    0x07, 0x00, 0x50, 0x04, 0x08, // 2090
    0x00, 0x60, 0x04, 0x00, 0x20, // 2100
];

/// 農曆年的新年比西曆年晚，這個陣列儲存西曆年和農曆年開始(該年第一天)的偏差量(天數)。此處有西元1901~2100年的資料。
pub(crate) const NEW_YEAR_DIFFERENCE: [u8; 200] = [
    49, 38, 28, 46, 34, 24, 43, 32, 21, 40, // 1910
    29, 48, 36, 25, 44, 33, 22, 41, 31, 50, // 1920
    38, 27, 46, 35, 23, 43, 32, 22, 40, 29, // 1930
    47, 36, 25, 44, 34, 23, 41, 30, 49, 38, // 1940
    26, 45, 35, 24, 43, 32, 21, 40, 28, 47, // 1950
    36, 26, 44, 33, 23, 42, 30, 48, 38, 27, // 1960
    45, 35, 24, 43, 32, 20, 39, 29, 47, 36, // 1970
    26, 45, 33, 22, 41, 30, 48, 37, 27, 46, // 1980
    35, 24, 43, 32, 50, 39, 28, 47, 36, 26, // 1990
    45, 34, 22, 40, 30, 49, 37, 27, 46, 35, // 2000
    23, 42, 31, 21, 39, 28, 48, 37, 25, 44, // 2010
    33, 22, 40, 30, 49, 38, 27, 46, 35, 24, // 2020
    42, 31, 21, 40, 28, 47, 36, 25, 43, 33, // 2030
    22, 41, 30, 49, 38, 27, 45, 34, 23, 42, // 2040
    31, 21, 40, 29, 47, 36, 25, 44, 32, 22, // 2050
    41, 31, 49, 38, 27, 45, 34, 23, 42, 32, // 2060
    20, 39, 28, 47, 35, 25, 44, 33, 22, 41, // 2070
    30, 49, 37, 26, 45, 35, 23, 42, 32, 21, // 2080
    39, 28, 47, 36, 25, 44, 33, 23, 40, 29, // 2090
    48, 37, 26, 45, 35, 24, 42, 31, 20, 39, // 2100
];

/// 從甲子年到癸亥年共六十年的八字重量。
pub(crate) const BA_ZI_WEIGHT_YEARS: [u8; 60] = [
    12, 9, 6, 7, 12, 5, 9, 8, 7, 8, 15, 9, 16, 8, 8, 19, 12, 6, 8, 7, 5, 15, 6, 16, 15, 7, 9, 12,
    10, 7, 15, 6, 5, 14, 14, 9, 7, 7, 9, 12, 8, 7, 13, 5, 14, 5, 9, 17, 5, 7, 12, 8, 8, 6, 19, 6,
    8, 16, 10, 7,
];

/// 從正月到臘月共十二個月的八字重量。
pub(crate) const BA_ZI_WEIGHT_MONTHS: [u8; 12] = [6, 7, 18, 9, 5, 16, 9, 15, 18, 8, 9, 5];

/// 從初一到三十共三十天的八字重量。
pub(crate) const BA_ZI_WEIGHT_DAYS: [u8; 30] = [
    5, 10, 8, 15, 16, 15, 8, 16, 8, 16, 9, 17, 8, 17, 10, 8, 9, 18, 5, 15, 10, 9, 8, 9, 15, 18, 7,
    8, 16, 6,
];

/// 從子時到亥時共十二地支的八字重量。
pub(crate) const BA_ZI_WEIGHT_TIME: [u8; 12] = [16, 6, 7, 10, 9, 16, 10, 8, 8, 9, 6, 6];
