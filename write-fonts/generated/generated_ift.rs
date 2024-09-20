// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

pub use read_fonts::tables::ift::{EntryFormatFlags, TablePatchFlags};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ift {
    Format1(PatchMapFormat1),
    Format2(PatchMapFormat2),
}

impl Ift {
    /// Construct a new `PatchMapFormat1` subtable
    #[allow(clippy::too_many_arguments)]
    pub fn format_1(
        compatibility_id: Vec<u32>,
        max_entry_index: u16,
        max_glyph_map_entry_index: u16,
        glyph_count: Uint24,
        glyph_map: GlyphMap,
        feature_map: Option<FeatureMap>,
        applied_entries_bitmap: Vec<u8>,
        uri_template_length: u16,
        uri_template: Vec<u8>,
        patch_encoding: u8,
    ) -> Self {
        Self::Format1(PatchMapFormat1::new(
            compatibility_id,
            max_entry_index,
            max_glyph_map_entry_index,
            glyph_count,
            glyph_map,
            feature_map,
            applied_entries_bitmap,
            uri_template_length,
            uri_template,
            patch_encoding,
        ))
    }

    /// Construct a new `PatchMapFormat2` subtable
    pub fn format_2(
        compatibility_id: Vec<u32>,
        default_patch_encoding: u8,
        entry_count: Uint24,
        entries: MappingEntries,
        entry_id_string_data: Option<IdStringData>,
        uri_template_length: u16,
        uri_template: Vec<u8>,
    ) -> Self {
        Self::Format2(PatchMapFormat2::new(
            compatibility_id,
            default_patch_encoding,
            entry_count,
            entries,
            entry_id_string_data,
            uri_template_length,
            uri_template,
        ))
    }
}

impl Default for Ift {
    fn default() -> Self {
        Self::Format1(Default::default())
    }
}

impl FontWrite for Ift {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format1(item) => item.table_type(),
            Self::Format2(item) => item.table_type(),
        }
    }
}

impl Validate for Ift {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::ift::Ift<'_>> for Ift {
    fn from_obj_ref(obj: &read_fonts::tables::ift::Ift, _: FontData) -> Self {
        use read_fonts::tables::ift::Ift as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => Ift::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => Ift::Format2(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::ift::Ift<'_>> for Ift {}

impl<'a> FontRead<'a> for Ift {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::Ift as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl From<PatchMapFormat1> for Ift {
    fn from(src: PatchMapFormat1) -> Ift {
        Ift::Format1(src)
    }
}

impl From<PatchMapFormat2> for Ift {
    fn from(src: PatchMapFormat2) -> Ift {
        Ift::Format2(src)
    }
}

/// [Patch Map Format Format 1](https://w3c.github.io/IFT/Overview.html#patch-map-format-1)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatchMapFormat1 {
    /// Unique ID that identifies compatible patches.
    pub compatibility_id: Vec<u32>,
    /// Largest entry index which appears in either the glyph map or feature map.
    pub max_entry_index: u16,
    /// Largest entry index which appears in the glyph map.
    pub max_glyph_map_entry_index: u16,
    pub glyph_count: Uint24,
    /// Sub table that maps glyph ids to entry indices.
    pub glyph_map: OffsetMarker<GlyphMap, WIDTH_32>,
    /// Sub table that maps feature and glyph ids to entry indices.
    pub feature_map: NullableOffsetMarker<FeatureMap, WIDTH_32>,
    pub applied_entries_bitmap: Vec<u8>,
    pub uri_template_length: u16,
    pub uri_template: Vec<u8>,
    /// Patch format number for patches referenced by this mapping.
    pub patch_encoding: u8,
}

impl PatchMapFormat1 {
    /// Construct a new `PatchMapFormat1`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        compatibility_id: Vec<u32>,
        max_entry_index: u16,
        max_glyph_map_entry_index: u16,
        glyph_count: Uint24,
        glyph_map: GlyphMap,
        feature_map: Option<FeatureMap>,
        applied_entries_bitmap: Vec<u8>,
        uri_template_length: u16,
        uri_template: Vec<u8>,
        patch_encoding: u8,
    ) -> Self {
        Self {
            compatibility_id: compatibility_id.into_iter().map(Into::into).collect(),
            max_entry_index,
            max_glyph_map_entry_index,
            glyph_count,
            glyph_map: glyph_map.into(),
            feature_map: feature_map.into(),
            applied_entries_bitmap: applied_entries_bitmap.into_iter().map(Into::into).collect(),
            uri_template_length,
            uri_template: uri_template.into_iter().map(Into::into).collect(),
            patch_encoding,
        }
    }
}

impl FontWrite for PatchMapFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u8).write_into(writer);
        (0 as u32).write_into(writer);
        self.compatibility_id.write_into(writer);
        self.max_entry_index.write_into(writer);
        self.max_glyph_map_entry_index.write_into(writer);
        self.glyph_count.write_into(writer);
        self.glyph_map.write_into(writer);
        self.feature_map.write_into(writer);
        self.applied_entries_bitmap.write_into(writer);
        self.uri_template_length.write_into(writer);
        self.uri_template.write_into(writer);
        self.patch_encoding.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("PatchMapFormat1")
    }
}

impl Validate for PatchMapFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PatchMapFormat1", |ctx| {
            ctx.in_field("glyph_map", |ctx| {
                self.glyph_map.validate_impl(ctx);
            });
            ctx.in_field("feature_map", |ctx| {
                self.feature_map.validate_impl(ctx);
            });
            ctx.in_field("uri_template", |ctx| {
                if self.uri_template.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::ift::PatchMapFormat1<'a>> for PatchMapFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::ift::PatchMapFormat1<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        PatchMapFormat1 {
            compatibility_id: obj.compatibility_id().to_owned_obj(offset_data),
            max_entry_index: obj.max_entry_index(),
            max_glyph_map_entry_index: obj.max_glyph_map_entry_index(),
            glyph_count: obj.glyph_count(),
            glyph_map: obj.glyph_map().to_owned_table(),
            feature_map: obj.feature_map().to_owned_table(),
            applied_entries_bitmap: obj.applied_entries_bitmap().to_owned_obj(offset_data),
            uri_template_length: obj.uri_template_length(),
            uri_template: obj.uri_template().to_owned_obj(offset_data),
            patch_encoding: obj.patch_encoding(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::PatchMapFormat1<'a>> for PatchMapFormat1 {}

impl<'a> FontRead<'a> for PatchMapFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::PatchMapFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlyphMap {
    pub first_mapped_glyph: u16,
}

impl GlyphMap {
    /// Construct a new `GlyphMap`
    pub fn new(first_mapped_glyph: u16) -> Self {
        Self { first_mapped_glyph }
    }
}

impl FontWrite for GlyphMap {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.first_mapped_glyph.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("GlyphMap")
    }
}

impl Validate for GlyphMap {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::GlyphMap<'a>> for GlyphMap {
    fn from_obj_ref(obj: &read_fonts::tables::ift::GlyphMap<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        GlyphMap {
            first_mapped_glyph: obj.first_mapped_glyph(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::GlyphMap<'a>> for GlyphMap {}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureMap {
    pub feature_count: u16,
    pub entry_map_data: Vec<u8>,
}

impl FeatureMap {
    /// Construct a new `FeatureMap`
    pub fn new(feature_count: u16, entry_map_data: Vec<u8>) -> Self {
        Self {
            feature_count,
            entry_map_data: entry_map_data.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for FeatureMap {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.feature_count.write_into(writer);
        self.entry_map_data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FeatureMap")
    }
}

impl Validate for FeatureMap {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::FeatureMap<'a>> for FeatureMap {
    fn from_obj_ref(obj: &read_fonts::tables::ift::FeatureMap<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        FeatureMap {
            feature_count: obj.feature_count(),
            entry_map_data: obj.entry_map_data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::FeatureMap<'a>> for FeatureMap {}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureRecord {
    pub feature_tag: Tag,
}

impl FeatureRecord {
    /// Construct a new `FeatureRecord`
    pub fn new(feature_tag: Tag) -> Self {
        Self { feature_tag }
    }
}

impl FontWrite for FeatureRecord {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.feature_tag.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FeatureRecord")
    }
}

impl Validate for FeatureRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::ift::FeatureRecord> for FeatureRecord {
    fn from_obj_ref(obj: &read_fonts::tables::ift::FeatureRecord, offset_data: FontData) -> Self {
        FeatureRecord {
            feature_tag: obj.feature_tag(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryMapRecord {}

impl EntryMapRecord {
    /// Construct a new `EntryMapRecord`
    pub fn new() -> Self {
        Self {}
    }
}

impl FontWrite for EntryMapRecord {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {}
    fn table_type(&self) -> TableType {
        TableType::Named("EntryMapRecord")
    }
}

impl Validate for EntryMapRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::ift::EntryMapRecord> for EntryMapRecord {
    fn from_obj_ref(obj: &read_fonts::tables::ift::EntryMapRecord, offset_data: FontData) -> Self {
        EntryMapRecord {}
    }
}

/// [Patch Map Format Format 2](https://w3c.github.io/IFT/Overview.html#patch-map-format-2)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatchMapFormat2 {
    /// Unique ID that identifies compatible patches.
    pub compatibility_id: Vec<u32>,
    /// Patch format number for patches referenced by this mapping.
    pub default_patch_encoding: u8,
    pub entry_count: Uint24,
    pub entries: OffsetMarker<MappingEntries, WIDTH_32>,
    pub entry_id_string_data: NullableOffsetMarker<IdStringData, WIDTH_32>,
    pub uri_template_length: u16,
    pub uri_template: Vec<u8>,
}

impl PatchMapFormat2 {
    /// Construct a new `PatchMapFormat2`
    pub fn new(
        compatibility_id: Vec<u32>,
        default_patch_encoding: u8,
        entry_count: Uint24,
        entries: MappingEntries,
        entry_id_string_data: Option<IdStringData>,
        uri_template_length: u16,
        uri_template: Vec<u8>,
    ) -> Self {
        Self {
            compatibility_id: compatibility_id.into_iter().map(Into::into).collect(),
            default_patch_encoding,
            entry_count,
            entries: entries.into(),
            entry_id_string_data: entry_id_string_data.into(),
            uri_template_length,
            uri_template: uri_template.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for PatchMapFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u8).write_into(writer);
        (0 as u32).write_into(writer);
        self.compatibility_id.write_into(writer);
        self.default_patch_encoding.write_into(writer);
        self.entry_count.write_into(writer);
        self.entries.write_into(writer);
        self.entry_id_string_data.write_into(writer);
        self.uri_template_length.write_into(writer);
        self.uri_template.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("PatchMapFormat2")
    }
}

impl Validate for PatchMapFormat2 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PatchMapFormat2", |ctx| {
            ctx.in_field("entries", |ctx| {
                self.entries.validate_impl(ctx);
            });
            ctx.in_field("entry_id_string_data", |ctx| {
                self.entry_id_string_data.validate_impl(ctx);
            });
            ctx.in_field("uri_template", |ctx| {
                if self.uri_template.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::ift::PatchMapFormat2<'a>> for PatchMapFormat2 {
    fn from_obj_ref(obj: &read_fonts::tables::ift::PatchMapFormat2<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        PatchMapFormat2 {
            compatibility_id: obj.compatibility_id().to_owned_obj(offset_data),
            default_patch_encoding: obj.default_patch_encoding(),
            entry_count: obj.entry_count(),
            entries: obj.entries().to_owned_table(),
            entry_id_string_data: obj.entry_id_string_data().to_owned_table(),
            uri_template_length: obj.uri_template_length(),
            uri_template: obj.uri_template().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::PatchMapFormat2<'a>> for PatchMapFormat2 {}

impl<'a> FontRead<'a> for PatchMapFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::PatchMapFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MappingEntries {
    pub entry_data: Vec<u8>,
}

impl MappingEntries {
    /// Construct a new `MappingEntries`
    pub fn new(entry_data: Vec<u8>) -> Self {
        Self {
            entry_data: entry_data.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for MappingEntries {
    fn write_into(&self, writer: &mut TableWriter) {
        self.entry_data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("MappingEntries")
    }
}

impl Validate for MappingEntries {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::MappingEntries<'a>> for MappingEntries {
    fn from_obj_ref(obj: &read_fonts::tables::ift::MappingEntries<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        MappingEntries {
            entry_data: obj.entry_data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::MappingEntries<'a>> for MappingEntries {}

impl<'a> FontRead<'a> for MappingEntries {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::MappingEntries as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryData {
    pub format_flags: EntryFormatFlags,
    pub feature_count: Option<u8>,
    pub feature_tags: Option<Vec<Tag>>,
    pub design_space_count: Option<u16>,
    pub design_space_segments: Option<Vec<DesignSpaceSegment>>,
    pub copy_count: Option<u8>,
    pub copy_indices: Option<Vec<Uint24>>,
    pub patch_encoding: Option<u8>,
    pub codepoint_data: Vec<u8>,
}

impl EntryData {
    /// Construct a new `EntryData`
    pub fn new(format_flags: EntryFormatFlags, codepoint_data: Vec<u8>) -> Self {
        Self {
            format_flags,
            codepoint_data: codepoint_data.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl FontWrite for EntryData {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.format_flags.write_into(writer);
        self.format_flags
            .contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE)
            .then(|| {
                self.feature_count
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.format_flags
            .contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE)
            .then(|| {
                self.feature_tags
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.format_flags
            .contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE)
            .then(|| {
                self.design_space_count
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.format_flags
            .contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE)
            .then(|| {
                self.design_space_segments
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.format_flags
            .contains(EntryFormatFlags::COPY_INDICES)
            .then(|| {
                self.copy_count
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.format_flags
            .contains(EntryFormatFlags::COPY_INDICES)
            .then(|| {
                self.copy_indices
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.format_flags
            .contains(EntryFormatFlags::PATCH_ENCODING)
            .then(|| {
                self.patch_encoding
                    .as_ref()
                    .expect("missing conditional field should have failed validation")
                    .write_into(writer)
            });
        self.codepoint_data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("EntryData")
    }
}

impl Validate for EntryData {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("EntryData", |ctx| {
            let format_flags = self.format_flags;
            ctx.in_field("feature_count", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.feature_count.is_some()
                {
                    ctx.report("'feature_count' is present but FEATURES_AND_DESIGN_SPACE not set")
                }
                if (format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.feature_count.is_none()
                {
                    ctx.report("FEATURES_AND_DESIGN_SPACE is set but 'feature_count' is None")
                }
            });
            ctx.in_field("feature_tags", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.feature_tags.is_some()
                {
                    ctx.report("'feature_tags' is present but FEATURES_AND_DESIGN_SPACE not set")
                }
                if (format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.feature_tags.is_none()
                {
                    ctx.report("FEATURES_AND_DESIGN_SPACE is set but 'feature_tags' is None")
                }
                if self.feature_tags.is_some()
                    && self.feature_tags.as_ref().unwrap().len() > (u8::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
            });
            ctx.in_field("design_space_count", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.design_space_count.is_some()
                {
                    ctx.report(
                        "'design_space_count' is present but FEATURES_AND_DESIGN_SPACE not set",
                    )
                }
                if (format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.design_space_count.is_none()
                {
                    ctx.report("FEATURES_AND_DESIGN_SPACE is set but 'design_space_count' is None")
                }
            });
            ctx.in_field("design_space_segments", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.design_space_segments.is_some()
                {
                    ctx.report(
                        "'design_space_segments' is present but FEATURES_AND_DESIGN_SPACE not set",
                    )
                }
                if (format_flags.contains(EntryFormatFlags::FEATURES_AND_DESIGN_SPACE))
                    && self.design_space_segments.is_none()
                {
                    ctx.report(
                        "FEATURES_AND_DESIGN_SPACE is set but 'design_space_segments' is None",
                    )
                }
                if self.design_space_segments.is_some()
                    && self.design_space_segments.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
                self.design_space_segments.validate_impl(ctx);
            });
            ctx.in_field("copy_count", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::COPY_INDICES))
                    && self.copy_count.is_some()
                {
                    ctx.report("'copy_count' is present but COPY_INDICES not set")
                }
                if (format_flags.contains(EntryFormatFlags::COPY_INDICES))
                    && self.copy_count.is_none()
                {
                    ctx.report("COPY_INDICES is set but 'copy_count' is None")
                }
            });
            ctx.in_field("copy_indices", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::COPY_INDICES))
                    && self.copy_indices.is_some()
                {
                    ctx.report("'copy_indices' is present but COPY_INDICES not set")
                }
                if (format_flags.contains(EntryFormatFlags::COPY_INDICES))
                    && self.copy_indices.is_none()
                {
                    ctx.report("COPY_INDICES is set but 'copy_indices' is None")
                }
                if self.copy_indices.is_some()
                    && self.copy_indices.as_ref().unwrap().len() > (u8::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
            });
            ctx.in_field("patch_encoding", |ctx| {
                if !(format_flags.contains(EntryFormatFlags::PATCH_ENCODING))
                    && self.patch_encoding.is_some()
                {
                    ctx.report("'patch_encoding' is present but PATCH_ENCODING not set")
                }
                if (format_flags.contains(EntryFormatFlags::PATCH_ENCODING))
                    && self.patch_encoding.is_none()
                {
                    ctx.report("PATCH_ENCODING is set but 'patch_encoding' is None")
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::ift::EntryData<'a>> for EntryData {
    fn from_obj_ref(obj: &read_fonts::tables::ift::EntryData<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        EntryData {
            format_flags: obj.format_flags(),
            feature_count: obj.feature_count(),
            feature_tags: obj.feature_tags().to_owned_obj(offset_data),
            design_space_count: obj.design_space_count(),
            design_space_segments: obj.design_space_segments().to_owned_obj(offset_data),
            copy_count: obj.copy_count(),
            copy_indices: obj.copy_indices().to_owned_obj(offset_data),
            patch_encoding: obj.patch_encoding(),
            codepoint_data: obj.codepoint_data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::EntryData<'a>> for EntryData {}

impl FontWrite for EntryFormatFlags {
    fn write_into(&self, writer: &mut TableWriter) {
        writer.write_slice(&self.bits().to_be_bytes())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DesignSpaceSegment {
    pub axis_tag: Tag,
    pub start: Fixed,
    pub end: Fixed,
}

impl DesignSpaceSegment {
    /// Construct a new `DesignSpaceSegment`
    pub fn new(axis_tag: Tag, start: Fixed, end: Fixed) -> Self {
        Self {
            axis_tag,
            start,
            end,
        }
    }
}

impl FontWrite for DesignSpaceSegment {
    fn write_into(&self, writer: &mut TableWriter) {
        self.axis_tag.write_into(writer);
        self.start.write_into(writer);
        self.end.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("DesignSpaceSegment")
    }
}

impl Validate for DesignSpaceSegment {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::ift::DesignSpaceSegment> for DesignSpaceSegment {
    fn from_obj_ref(obj: &read_fonts::tables::ift::DesignSpaceSegment, _: FontData) -> Self {
        DesignSpaceSegment {
            axis_tag: obj.axis_tag(),
            start: obj.start(),
            end: obj.end(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdStringData {
    pub id_data: Vec<u8>,
}

impl IdStringData {
    /// Construct a new `IdStringData`
    pub fn new(id_data: Vec<u8>) -> Self {
        Self {
            id_data: id_data.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for IdStringData {
    fn write_into(&self, writer: &mut TableWriter) {
        self.id_data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("IdStringData")
    }
}

impl Validate for IdStringData {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::IdStringData<'a>> for IdStringData {
    fn from_obj_ref(obj: &read_fonts::tables::ift::IdStringData<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        IdStringData {
            id_data: obj.id_data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::IdStringData<'a>> for IdStringData {}

impl<'a> FontRead<'a> for IdStringData {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::IdStringData as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Per Table Brotli Patch](https://w3c.github.io/IFT/Overview.html#per-table-brotli)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableKeyedPatch {
    pub format: Tag,
    /// Unique ID that identifies compatible patches.
    pub compatibility_id: Vec<u32>,
    pub patches_count: u16,
    pub patchs: Vec<OffsetMarker<TablePatch, WIDTH_32>>,
}

impl TableKeyedPatch {
    /// Construct a new `TableKeyedPatch`
    pub fn new(
        format: Tag,
        compatibility_id: Vec<u32>,
        patches_count: u16,
        patchs: Vec<TablePatch>,
    ) -> Self {
        Self {
            format,
            compatibility_id: compatibility_id.into_iter().map(Into::into).collect(),
            patches_count,
            patchs: patchs.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for TableKeyedPatch {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.format.write_into(writer);
        (0 as u32).write_into(writer);
        self.compatibility_id.write_into(writer);
        self.patches_count.write_into(writer);
        self.patchs.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("TableKeyedPatch")
    }
}

impl Validate for TableKeyedPatch {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("TableKeyedPatch", |ctx| {
            ctx.in_field("patchs", |ctx| {
                self.patchs.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::ift::TableKeyedPatch<'a>> for TableKeyedPatch {
    fn from_obj_ref(obj: &read_fonts::tables::ift::TableKeyedPatch<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        TableKeyedPatch {
            format: obj.format(),
            compatibility_id: obj.compatibility_id().to_owned_obj(offset_data),
            patches_count: obj.patches_count(),
            patchs: obj.patchs().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::TableKeyedPatch<'a>> for TableKeyedPatch {}

impl<'a> FontRead<'a> for TableKeyedPatch {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::TableKeyedPatch as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [TablePatch](https://w3c.github.io/IFT/Overview.html#tablepatch)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TablePatch {
    pub tag: Tag,
    pub flags: TablePatchFlags,
    pub max_uncompressed_length: u32,
    pub brotli_stream: Vec<u8>,
}

impl TablePatch {
    /// Construct a new `TablePatch`
    pub fn new(
        tag: Tag,
        flags: TablePatchFlags,
        max_uncompressed_length: u32,
        brotli_stream: Vec<u8>,
    ) -> Self {
        Self {
            tag,
            flags,
            max_uncompressed_length,
            brotli_stream: brotli_stream.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for TablePatch {
    fn write_into(&self, writer: &mut TableWriter) {
        self.tag.write_into(writer);
        self.flags.write_into(writer);
        self.max_uncompressed_length.write_into(writer);
        self.brotli_stream.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("TablePatch")
    }
}

impl Validate for TablePatch {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::TablePatch<'a>> for TablePatch {
    fn from_obj_ref(obj: &read_fonts::tables::ift::TablePatch<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        TablePatch {
            tag: obj.tag(),
            flags: obj.flags(),
            max_uncompressed_length: obj.max_uncompressed_length(),
            brotli_stream: obj.brotli_stream().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::TablePatch<'a>> for TablePatch {}

impl<'a> FontRead<'a> for TablePatch {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::TablePatch as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl FontWrite for TablePatchFlags {
    fn write_into(&self, writer: &mut TableWriter) {
        writer.write_slice(&self.bits().to_be_bytes())
    }
}
