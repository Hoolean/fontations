// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::parse_prelude::*;

/// [GDEF](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#gdef-header) 1.0
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct GdefMarker {
    mark_glyph_sets_def_offset_byte_start: Option<usize>,
    item_var_store_offset_byte_start: Option<usize>,
}

impl GdefMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn glyph_class_def_offset_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn attach_list_offset_byte_range(&self) -> Range<usize> {
        let start = self.glyph_class_def_offset_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn lig_caret_list_offset_byte_range(&self) -> Range<usize> {
        let start = self.attach_list_offset_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn mark_attach_class_def_offset_byte_range(&self) -> Range<usize> {
        let start = self.lig_caret_list_offset_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn mark_glyph_sets_def_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.mark_glyph_sets_def_offset_byte_start?;
        Some(start..start + Offset16::RAW_BYTE_LEN)
    }
    fn item_var_store_offset_byte_range(&self) -> Option<Range<usize>> {
        let start = self.item_var_store_offset_byte_start?;
        Some(start..start + Offset32::RAW_BYTE_LEN)
    }
}

impl TableInfo for GdefMarker {
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        let version: MajorMinor = cursor.read()?;
        cursor.advance::<Offset16>();
        cursor.advance::<Offset16>();
        cursor.advance::<Offset16>();
        cursor.advance::<Offset16>();
        let mark_glyph_sets_def_offset_byte_start = version
            .compatible(MajorMinor::VERSION_1_2)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(MajorMinor::VERSION_1_2)
            .then(|| cursor.advance::<Offset16>());
        let item_var_store_offset_byte_start = version
            .compatible(MajorMinor::VERSION_1_3)
            .then(|| cursor.position())
            .transpose()?;
        version
            .compatible(MajorMinor::VERSION_1_3)
            .then(|| cursor.advance::<Offset32>());
        cursor.finish(GdefMarker {
            mark_glyph_sets_def_offset_byte_start,
            item_var_store_offset_byte_start,
        })
    }
}

/// [GDEF](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#gdef-header) 1.0
pub type Gdef<'a> = TableRef<'a, GdefMarker>;

impl<'a> Gdef<'a> {
    /// The major/minor version of the GDEF table
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset to class definition table for glyph type, from beginning
    /// of GDEF header (may be NULL)
    pub fn glyph_class_def_offset(&self) -> Nullable<Offset16> {
        let range = self.shape.glyph_class_def_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`glyph_class_def_offset`][Self::glyph_class_def_offset].
    pub fn glyph_class_def(&self) -> Option<Result<ClassDef<'a>, ReadError>> {
        let data = &self.data;
        self.glyph_class_def_offset().resolve(data)
    }

    /// Offset to attachment point list table, from beginning of GDEF
    /// header (may be NULL)
    pub fn attach_list_offset(&self) -> Nullable<Offset16> {
        let range = self.shape.attach_list_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`attach_list_offset`][Self::attach_list_offset].
    pub fn attach_list(&self) -> Option<Result<AttachList<'a>, ReadError>> {
        let data = &self.data;
        self.attach_list_offset().resolve(data)
    }

    /// Offset to ligature caret list table, from beginning of GDEF
    /// header (may be NULL)
    pub fn lig_caret_list_offset(&self) -> Nullable<Offset16> {
        let range = self.shape.lig_caret_list_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`lig_caret_list_offset`][Self::lig_caret_list_offset].
    pub fn lig_caret_list(&self) -> Option<Result<LigCaretList<'a>, ReadError>> {
        let data = &self.data;
        self.lig_caret_list_offset().resolve(data)
    }

    /// Offset to class definition table for mark attachment type, from
    /// beginning of GDEF header (may be NULL)
    pub fn mark_attach_class_def_offset(&self) -> Nullable<Offset16> {
        let range = self.shape.mark_attach_class_def_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`mark_attach_class_def_offset`][Self::mark_attach_class_def_offset].
    pub fn mark_attach_class_def(&self) -> Option<Result<ClassDef<'a>, ReadError>> {
        let data = &self.data;
        self.mark_attach_class_def_offset().resolve(data)
    }

    /// Offset to the table of mark glyph set definitions, from
    /// beginning of GDEF header (may be NULL)
    pub fn mark_glyph_sets_def_offset(&self) -> Option<Nullable<Offset16>> {
        let range = self.shape.mark_glyph_sets_def_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`mark_glyph_sets_def_offset`][Self::mark_glyph_sets_def_offset].
    pub fn mark_glyph_sets_def(&self) -> Option<Result<MarkGlyphSets<'a>, ReadError>> {
        let data = &self.data;
        self.mark_glyph_sets_def_offset()?.resolve(data)
    }

    /// Offset to the Item Variation Store table, from beginning of
    /// GDEF header (may be NULL)
    pub fn item_var_store_offset(&self) -> Option<Nullable<Offset32>> {
        let range = self.shape.item_var_store_offset_byte_range()?;
        Some(self.data.read_at(range.start).unwrap())
    }

    /// Attempt to resolve [`item_var_store_offset`][Self::item_var_store_offset].
    pub fn item_var_store(&self) -> Option<Result<ClassDef<'a>, ReadError>> {
        let data = &self.data;
        self.item_var_store_offset()?.resolve(data)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Gdef<'a> {
    fn type_name(&self) -> &str {
        "Gdef"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("glyph_class_def_offset", self.glyph_class_def())),
            2usize => Some(Field::new("attach_list_offset", self.attach_list())),
            3usize => Some(Field::new("lig_caret_list_offset", self.lig_caret_list())),
            4usize => Some(Field::new(
                "mark_attach_class_def_offset",
                self.mark_attach_class_def(),
            )),
            5usize => Some(Field::new(
                "mark_glyph_sets_def_offset",
                self.mark_glyph_sets_def(),
            )),
            6usize => Some(Field::new("item_var_store_offset", self.item_var_store())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Gdef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Used in the [Glyph Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GlyphClassDef {
    Base = 1,
    Ligature = 2,
    Mark = 3,
    Component = 4,
    Unknown,
}

impl GlyphClassDef {
    #[doc = r" Create from a raw scalar."]
    #[doc = r""]
    #[doc = r" This will never fail; unknown values will be mapped to the `Unknown` variant"]
    pub fn new(raw: u16) -> Self {
        match raw {
            1 => Self::Base,
            2 => Self::Ligature,
            3 => Self::Mark,
            4 => Self::Component,
            _ => Self::Unknown,
        }
    }
}

impl font_types::Scalar for GlyphClassDef {
    type Raw = <u16 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        (self as u16).to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self::new(t)
    }
}

#[cfg(feature = "traversal")]
impl<'a> From<GlyphClassDef> for FieldType<'a> {
    fn from(src: GlyphClassDef) -> FieldType<'a> {
        (src as u16).into()
    }
}

/// [Attachment Point List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#attachment-point-list-table)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct AttachListMarker {
    attach_point_offsets_byte_len: usize,
}

impl AttachListMarker {
    fn coverage_offset_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn glyph_count_byte_range(&self) -> Range<usize> {
        let start = self.coverage_offset_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn attach_point_offsets_byte_range(&self) -> Range<usize> {
        let start = self.glyph_count_byte_range().end;
        start..start + self.attach_point_offsets_byte_len
    }
}

impl TableInfo for AttachListMarker {
    #[allow(unused_parens)]
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<Offset16>();
        let glyph_count: u16 = cursor.read()?;
        let attach_point_offsets_byte_len = glyph_count as usize * Offset16::RAW_BYTE_LEN;
        cursor.advance_by(attach_point_offsets_byte_len);
        cursor.finish(AttachListMarker {
            attach_point_offsets_byte_len,
        })
    }
}

/// [Attachment Point List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#attachment-point-list-table)
pub type AttachList<'a> = TableRef<'a, AttachListMarker>;

impl<'a> AttachList<'a> {
    /// Offset to Coverage table - from beginning of AttachList table
    pub fn coverage_offset(&self) -> Offset16 {
        let range = self.shape.coverage_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`coverage_offset`][Self::coverage_offset].
    pub fn coverage(&self) -> Result<CoverageTable<'a>, ReadError> {
        let data = &self.data;
        self.coverage_offset().resolve(data)
    }

    /// Number of glyphs with attachment points
    pub fn glyph_count(&self) -> u16 {
        let range = self.shape.glyph_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of offsets to AttachPoint tables-from beginning of
    /// AttachList table-in Coverage Index order
    pub fn attach_point_offsets(&self) -> &'a [BigEndian<Offset16>] {
        let range = self.shape.attach_point_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    pub fn attach_point(&self) -> impl Iterator<Item = Result<AttachPoint<'a>, ReadError>> + 'a {
        let data = self.data.clone();
        self.attach_point_offsets()
            .iter()
            .map(move |off| off.get().resolve(&data))
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for AttachList<'a> {
    fn type_name(&self) -> &str {
        "AttachList"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("coverage_offset", self.coverage())),
            1usize => Some(Field::new("glyph_count", self.glyph_count())),
            2usize => Some({
                let this = self.sneaky_copy();
                Field::new(
                    "attach_point_offsets",
                    FieldType::offset_iter(move || {
                        Box::new(this.attach_point().map(|item| item.into()))
                            as Box<dyn Iterator<Item = FieldType<'a>> + 'a>
                    }),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for AttachList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Part of [AttachList]
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct AttachPointMarker {
    point_indices_byte_len: usize,
}

impl AttachPointMarker {
    fn point_count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn point_indices_byte_range(&self) -> Range<usize> {
        let start = self.point_count_byte_range().end;
        start..start + self.point_indices_byte_len
    }
}

impl TableInfo for AttachPointMarker {
    #[allow(unused_parens)]
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        let point_count: u16 = cursor.read()?;
        let point_indices_byte_len = point_count as usize * u16::RAW_BYTE_LEN;
        cursor.advance_by(point_indices_byte_len);
        cursor.finish(AttachPointMarker {
            point_indices_byte_len,
        })
    }
}

/// Part of [AttachList]
pub type AttachPoint<'a> = TableRef<'a, AttachPointMarker>;

impl<'a> AttachPoint<'a> {
    /// Number of attachment points on this glyph
    pub fn point_count(&self) -> u16 {
        let range = self.shape.point_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of contour point indices -in increasing numerical order
    pub fn point_indices(&self) -> &'a [BigEndian<u16>] {
        let range = self.shape.point_indices_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for AttachPoint<'a> {
    fn type_name(&self) -> &str {
        "AttachPoint"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("point_count", self.point_count())),
            1usize => Some(Field::new("point_indices", self.point_indices())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for AttachPoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// [Ligature Caret List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-caret-list-table)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct LigCaretListMarker {
    lig_glyph_offsets_byte_len: usize,
}

impl LigCaretListMarker {
    fn coverage_offset_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn lig_glyph_count_byte_range(&self) -> Range<usize> {
        let start = self.coverage_offset_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn lig_glyph_offsets_byte_range(&self) -> Range<usize> {
        let start = self.lig_glyph_count_byte_range().end;
        start..start + self.lig_glyph_offsets_byte_len
    }
}

impl TableInfo for LigCaretListMarker {
    #[allow(unused_parens)]
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<Offset16>();
        let lig_glyph_count: u16 = cursor.read()?;
        let lig_glyph_offsets_byte_len = lig_glyph_count as usize * Offset16::RAW_BYTE_LEN;
        cursor.advance_by(lig_glyph_offsets_byte_len);
        cursor.finish(LigCaretListMarker {
            lig_glyph_offsets_byte_len,
        })
    }
}

/// [Ligature Caret List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-caret-list-table)
pub type LigCaretList<'a> = TableRef<'a, LigCaretListMarker>;

impl<'a> LigCaretList<'a> {
    /// Offset to Coverage table - from beginning of LigCaretList table
    pub fn coverage_offset(&self) -> Offset16 {
        let range = self.shape.coverage_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`coverage_offset`][Self::coverage_offset].
    pub fn coverage(&self) -> Result<CoverageTable<'a>, ReadError> {
        let data = &self.data;
        self.coverage_offset().resolve(data)
    }

    /// Number of ligature glyphs
    pub fn lig_glyph_count(&self) -> u16 {
        let range = self.shape.lig_glyph_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of offsets to LigGlyph tables, from beginning of
    /// LigCaretList table —in Coverage Index order
    pub fn lig_glyph_offsets(&self) -> &'a [BigEndian<Offset16>] {
        let range = self.shape.lig_glyph_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    pub fn lig_glyph(&self) -> impl Iterator<Item = Result<LigGlyph<'a>, ReadError>> + 'a {
        let data = self.data.clone();
        self.lig_glyph_offsets()
            .iter()
            .map(move |off| off.get().resolve(&data))
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for LigCaretList<'a> {
    fn type_name(&self) -> &str {
        "LigCaretList"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("coverage_offset", self.coverage())),
            1usize => Some(Field::new("lig_glyph_count", self.lig_glyph_count())),
            2usize => Some({
                let this = self.sneaky_copy();
                Field::new(
                    "lig_glyph_offsets",
                    FieldType::offset_iter(move || {
                        Box::new(this.lig_glyph().map(|item| item.into()))
                            as Box<dyn Iterator<Item = FieldType<'a>> + 'a>
                    }),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for LigCaretList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// [Ligature Glyph Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-glyph-table)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct LigGlyphMarker {
    caret_value_offsets_byte_len: usize,
}

impl LigGlyphMarker {
    fn caret_count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn caret_value_offsets_byte_range(&self) -> Range<usize> {
        let start = self.caret_count_byte_range().end;
        start..start + self.caret_value_offsets_byte_len
    }
}

impl TableInfo for LigGlyphMarker {
    #[allow(unused_parens)]
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        let caret_count: u16 = cursor.read()?;
        let caret_value_offsets_byte_len = caret_count as usize * Offset16::RAW_BYTE_LEN;
        cursor.advance_by(caret_value_offsets_byte_len);
        cursor.finish(LigGlyphMarker {
            caret_value_offsets_byte_len,
        })
    }
}

/// [Ligature Glyph Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-glyph-table)
pub type LigGlyph<'a> = TableRef<'a, LigGlyphMarker>;

impl<'a> LigGlyph<'a> {
    /// Number of CaretValue tables for this ligature (components - 1)
    pub fn caret_count(&self) -> u16 {
        let range = self.shape.caret_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of offsets to CaretValue tables, from beginning of
    /// LigGlyph table — in increasing coordinate order
    pub fn caret_value_offsets(&self) -> &'a [BigEndian<Offset16>] {
        let range = self.shape.caret_value_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    pub fn caret_value(&self) -> impl Iterator<Item = Result<CaretValue<'a>, ReadError>> + 'a {
        let data = self.data.clone();
        self.caret_value_offsets()
            .iter()
            .map(move |off| off.get().resolve(&data))
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for LigGlyph<'a> {
    fn type_name(&self) -> &str {
        "LigGlyph"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("caret_count", self.caret_count())),
            1usize => Some({
                let this = self.sneaky_copy();
                Field::new(
                    "caret_value_offsets",
                    FieldType::offset_iter(move || {
                        Box::new(this.caret_value().map(|item| item.into()))
                            as Box<dyn Iterator<Item = FieldType<'a>> + 'a>
                    }),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for LigGlyph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// [Caret Value Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caret-value-tables)
pub enum CaretValue<'a> {
    Format1(CaretValueFormat1<'a>),
    Format2(CaretValueFormat2<'a>),
    Format3(CaretValueFormat3<'a>),
}

impl<'a> FontRead<'a> for CaretValue<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let format: u16 = data.read_at(0)?;
        match format {
            CaretValueFormat1Marker::FORMAT => Ok(Self::Format1(FontRead::read(data)?)),
            CaretValueFormat2Marker::FORMAT => Ok(Self::Format2(FontRead::read(data)?)),
            CaretValueFormat3Marker::FORMAT => Ok(Self::Format3(FontRead::read(data)?)),
            other => Err(ReadError::InvalidFormat(other.into())),
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> CaretValue<'a> {
    fn dyn_inner<'b>(&'b self) -> &'b dyn SomeTable<'a> {
        match self {
            Self::Format1(table) => table,
            Self::Format2(table) => table,
            Self::Format3(table) => table,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for CaretValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dyn_inner().fmt(f)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for CaretValue<'a> {
    fn type_name(&self) -> &str {
        self.dyn_inner().type_name()
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.dyn_inner().get_field(idx)
    }
}

impl Format<u16> for CaretValueFormat1Marker {
    const FORMAT: u16 = 1;
}

/// [CaretValue Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-1)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CaretValueFormat1Marker {}

impl CaretValueFormat1Marker {
    fn caret_value_format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn coordinate_byte_range(&self) -> Range<usize> {
        let start = self.caret_value_format_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
}

impl TableInfo for CaretValueFormat1Marker {
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<i16>();
        cursor.finish(CaretValueFormat1Marker {})
    }
}

/// [CaretValue Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-1)
pub type CaretValueFormat1<'a> = TableRef<'a, CaretValueFormat1Marker>;

impl<'a> CaretValueFormat1<'a> {
    /// Format identifier: format = 1
    pub fn caret_value_format(&self) -> u16 {
        let range = self.shape.caret_value_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// X or Y value, in design units
    pub fn coordinate(&self) -> i16 {
        let range = self.shape.coordinate_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for CaretValueFormat1<'a> {
    fn type_name(&self) -> &str {
        "CaretValueFormat1"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("caret_value_format", self.caret_value_format())),
            1usize => Some(Field::new("coordinate", self.coordinate())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for CaretValueFormat1<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

impl Format<u16> for CaretValueFormat2Marker {
    const FORMAT: u16 = 2;
}

/// [CaretValue Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-2)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CaretValueFormat2Marker {}

impl CaretValueFormat2Marker {
    fn caret_value_format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn caret_value_point_index_byte_range(&self) -> Range<usize> {
        let start = self.caret_value_format_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
}

impl TableInfo for CaretValueFormat2Marker {
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.finish(CaretValueFormat2Marker {})
    }
}

/// [CaretValue Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-2)
pub type CaretValueFormat2<'a> = TableRef<'a, CaretValueFormat2Marker>;

impl<'a> CaretValueFormat2<'a> {
    /// Format identifier: format = 2
    pub fn caret_value_format(&self) -> u16 {
        let range = self.shape.caret_value_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Contour point index on glyph
    pub fn caret_value_point_index(&self) -> u16 {
        let range = self.shape.caret_value_point_index_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for CaretValueFormat2<'a> {
    fn type_name(&self) -> &str {
        "CaretValueFormat2"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("caret_value_format", self.caret_value_format())),
            1usize => Some(Field::new(
                "caret_value_point_index",
                self.caret_value_point_index(),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for CaretValueFormat2<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

impl Format<u16> for CaretValueFormat3Marker {
    const FORMAT: u16 = 3;
}

/// [CaretValue Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-3)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CaretValueFormat3Marker {}

impl CaretValueFormat3Marker {
    fn caret_value_format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn coordinate_byte_range(&self) -> Range<usize> {
        let start = self.caret_value_format_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }
    fn device_offset_byte_range(&self) -> Range<usize> {
        let start = self.coordinate_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
}

impl TableInfo for CaretValueFormat3Marker {
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<i16>();
        cursor.advance::<Offset16>();
        cursor.finish(CaretValueFormat3Marker {})
    }
}

/// [CaretValue Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-3)
pub type CaretValueFormat3<'a> = TableRef<'a, CaretValueFormat3Marker>;

impl<'a> CaretValueFormat3<'a> {
    /// Format identifier-format = 3
    pub fn caret_value_format(&self) -> u16 {
        let range = self.shape.caret_value_format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// X or Y value, in design units
    pub fn coordinate(&self) -> i16 {
        let range = self.shape.coordinate_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset to Device table (non-variable font) / Variation Index
    /// table (variable font) for X or Y value-from beginning of
    /// CaretValue table
    pub fn device_offset(&self) -> Offset16 {
        let range = self.shape.device_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`device_offset`][Self::device_offset].
    pub fn device(&self) -> Result<Device<'a>, ReadError> {
        let data = &self.data;
        self.device_offset().resolve(data)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for CaretValueFormat3<'a> {
    fn type_name(&self) -> &str {
        "CaretValueFormat3"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("caret_value_format", self.caret_value_format())),
            1usize => Some(Field::new("coordinate", self.coordinate())),
            2usize => Some(Field::new("device_offset", self.device())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for CaretValueFormat3<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

impl Format<u16> for MarkGlyphSetsMarker {
    const FORMAT: u16 = 1;
}

/// [Mark Glyph Sets Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct MarkGlyphSetsMarker {
    coverage_offsets_byte_len: usize,
}

impl MarkGlyphSetsMarker {
    fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }
    fn mark_glyph_set_count_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn coverage_offsets_byte_range(&self) -> Range<usize> {
        let start = self.mark_glyph_set_count_byte_range().end;
        start..start + self.coverage_offsets_byte_len
    }
}

impl TableInfo for MarkGlyphSetsMarker {
    #[allow(unused_parens)]
    fn parse(data: FontData) -> Result<TableRef<Self>, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        let mark_glyph_set_count: u16 = cursor.read()?;
        let coverage_offsets_byte_len = mark_glyph_set_count as usize * Offset32::RAW_BYTE_LEN;
        cursor.advance_by(coverage_offsets_byte_len);
        cursor.finish(MarkGlyphSetsMarker {
            coverage_offsets_byte_len,
        })
    }
}

/// [Mark Glyph Sets Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table)
pub type MarkGlyphSets<'a> = TableRef<'a, MarkGlyphSetsMarker>;

impl<'a> MarkGlyphSets<'a> {
    /// Format identifier == 1
    pub fn format(&self) -> u16 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of mark glyph sets defined
    pub fn mark_glyph_set_count(&self) -> u16 {
        let range = self.shape.mark_glyph_set_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Array of offsets to mark glyph set coverage tables, from the
    /// start of the MarkGlyphSets table.
    pub fn coverage_offsets(&self) -> &'a [BigEndian<Offset32>] {
        let range = self.shape.coverage_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    pub fn coverage(&self) -> impl Iterator<Item = Result<CoverageTable<'a>, ReadError>> + 'a {
        let data = self.data.clone();
        self.coverage_offsets()
            .iter()
            .map(move |off| off.get().resolve(&data))
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for MarkGlyphSets<'a> {
    fn type_name(&self) -> &str {
        "MarkGlyphSets"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new(
                "mark_glyph_set_count",
                self.mark_glyph_set_count(),
            )),
            2usize => Some({
                let this = self.sneaky_copy();
                Field::new(
                    "coverage_offsets",
                    FieldType::offset_iter(move || {
                        Box::new(this.coverage().map(|item| item.into()))
                            as Box<dyn Iterator<Item = FieldType<'a>> + 'a>
                    }),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for MarkGlyphSets<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
