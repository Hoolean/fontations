// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::parse_prelude::*;

/// The [hmtx (Horizontal Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct HmtxMarker {
    h_metrics_byte_len: usize,
    left_side_bearings_byte_len: usize,
}

impl HmtxMarker {
    fn h_metrics_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + self.h_metrics_byte_len
    }
    fn left_side_bearings_byte_range(&self) -> Range<usize> {
        let start = self.h_metrics_byte_range().end;
        start..start + self.left_side_bearings_byte_len
    }
}

impl ReadArgs for HmtxMarker {
    type Args = (u16, u16);
}

impl TableInfoWithArgs for HmtxMarker {
    #[allow(unused_parens)]
    fn parse_with_args<'a>(
        data: FontData<'a>,
        args: &(u16, u16),
    ) -> Result<TableRef<'a, Self>, ReadError> {
        let (number_of_h_metrics, num_glyphs) = *args;
        let mut cursor = data.cursor();
        let h_metrics_byte_len = number_of_h_metrics as usize * LongHorMetric::RAW_BYTE_LEN;
        cursor.advance_by(h_metrics_byte_len);
        let left_side_bearings_byte_len =
            num_glyphs.saturating_sub(number_of_h_metrics) as usize * i16::RAW_BYTE_LEN;
        cursor.advance_by(left_side_bearings_byte_len);
        cursor.finish(HmtxMarker {
            h_metrics_byte_len,
            left_side_bearings_byte_len,
        })
    }
}

/// The [hmtx (Horizontal Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) table
pub type Hmtx<'a> = TableRef<'a, HmtxMarker>;

impl<'a> Hmtx<'a> {
    /// Paired advance width and left side bearing values for each
    /// glyph. Records are indexed by glyph ID.
    pub fn h_metrics(&self) -> &'a [LongHorMetric] {
        let range = self.shape.h_metrics_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Left side bearings for glyph IDs greater than or equal to
    /// numberOfHMetrics.
    pub fn left_side_bearings(&self) -> &'a [BigEndian<i16>] {
        let range = self.shape.left_side_bearings_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Hmtx<'a> {
    fn type_name(&self) -> &str {
        "Hmtx"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new(
                "h_metrics",
                traversal::FieldType::array_of_records(
                    self.h_metrics(),
                    self.offset_data().clone(),
                ),
            )),
            1usize => Some(Field::new("left_side_bearings", self.left_side_bearings())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Hmtx<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct LongHorMetric {
    /// Advance width, in font design units.
    pub advance_width: BigEndian<u16>,
    /// Glyph left side bearing, in font design units.
    pub lsb: BigEndian<i16>,
}

impl LongHorMetric {
    /// Advance width, in font design units.
    pub fn advance_width(&self) -> u16 {
        self.advance_width.get()
    }

    /// Glyph left side bearing, in font design units.
    pub fn lsb(&self) -> i16 {
        self.lsb.get()
    }
}

impl FixedSized for LongHorMetric {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + i16::RAW_BYTE_LEN;
}

#[cfg(feature = "traversal")]
impl<'a> SomeRecord<'a> for LongHorMetric {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "LongHorMetric",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("advance_width", self.advance_width())),
                1usize => Some(Field::new("lsb", self.lsb())),
                _ => None,
            }),
            data,
        }
    }
}
