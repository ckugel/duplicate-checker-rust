use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use exif::{Exif, Field, In, Reader, Tag};

pub(crate) struct FileDataImg {
    size: u64,
    dimensions: [u16; 2],
    created_on: [u16; 6],
    exposure_time: u32,
}

impl FileDataImg {
    pub fn new(path: &str) -> FileDataImg {
        let reader: Reader = Reader::new();
        let file: File = File::open(path).unwrap();
        let exif: Exif = reader.read_from_container(&mut BufReader::new(&file)).unwrap();

        return FileDataImg {
            size: file.metadata().unwrap().len(),
            dimensions: [Self::get_width(&exif), Self::get_length(&exif)],
            created_on: Self::created_on(&exif),
            exposure_time: Self::get_exposure_time(&exif)
        };
    }

    fn get_as_string(exif:&Exif, tag: Tag) -> Option<String> {
        let field = exif.get_field(tag, In::PRIMARY);
        return match field {
            None => {
                None
            }
            Some(_) => {
                Some(field.unwrap().display_value().with_unit(exif).to_string().to_string())
            }
        }

    }

    fn parse_from_only_space(as_a_string: &String) -> &str {
        return &as_a_string[0..as_a_string.find(' ').unwrap()];
    }

    fn get_exposure_time(exif: &Exif) -> u32 {
        let as_a_string: String = Self::get_as_string(exif, Tag::ExposureTime).unwrap_or(String::from("/1 "));
        return u32::from_str(&as_a_string[as_a_string.find('/').unwrap() + 1.. as_a_string.rfind(' ').unwrap()]).unwrap();
    }

    fn get_width(exif: &Exif) -> u16 {
        let as_a_string: String = Self::get_as_string(exif, Tag::PixelXDimension).unwrap_or(String::from(" 1048"));
        return u16::from_str(Self::parse_from_only_space(&as_a_string)).unwrap();
    }

    fn get_length(exif: &Exif) -> u16 {
        let as_a_string: String = Self::get_as_string(exif, Tag::PixelYDimension).unwrap_or(String::from(" 2048"));
        return u16::from_str(Self::parse_from_only_space(&as_a_string)).unwrap();
    }

    fn created_on(exif: &Exif) -> [u16; 6] {
        let as_a_string: String = Self::get_as_string(exif, Tag::DateTime).unwrap_or(String::from("0000 00 00 00 00 00"));
        let as_a_str: &str = as_a_string.as_str();
        let mut arr: [u16; 6] = [0, 0, 0, 0, 0, 0];

        // this makes it faster cause it's a trust me bro to the compiler
        assert!(as_a_str.len() >= 19);
        arr[0] = u16::from_str(&as_a_str[0..4]).unwrap();
        arr[1] = u16::from_str(&as_a_str[5..7]).unwrap();
        arr[2] = u16::from_str(&as_a_str[8..10]).unwrap();
        arr[3] = u16::from_str(&as_a_str[11..13]).unwrap();
        arr[4] = u16::from_str(&as_a_str[14..16]).unwrap();
        arr[5] = u16::from_str(&as_a_str[17..]).unwrap();

        return arr;
    }

    pub fn hash<H: Hasher>(&self, state: &mut H) -> u64 {
        state.write_u64(self.size);
        state.write_u16(self.dimensions[0]);
        state.write_u16(self.dimensions[1]);
        for item in self.created_on {
            state.write_u16(item);
        }
        state.write_u32(self.exposure_time);
        return state.finish();
    }
}

impl PartialEq<Self> for FileDataImg {
    fn eq(&self, other: &Self) -> bool {
        let mut visitor: [bool; 2] = [true, true];
        visitor[(self.size == other.size) as usize] = false;
        visitor[(self.dimensions[0] == other.dimensions[0]) as usize] = false;
        visitor[(self.dimensions[1] == other.dimensions[1]) as usize] = false;
        for i in 0..self.created_on.len() {
            visitor[(self.created_on[i] == other.created_on[i]) as usize] = false;
        }
        return visitor[0];
    }
}
