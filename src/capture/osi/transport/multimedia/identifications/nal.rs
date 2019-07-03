/// Coded Slice of a Non-IDR Picture
pub const NON_IDR_SLICE: u8 = 1;
/// Coded Slice Data Partition A
pub const PART_A_SLICE: u8 = 2;
/// Coded Slice Data Partition B
pub const PART_B_SLICE: u8 = 3;
/// Coded Slice Data Partition C
pub const PART_C_SLICE: u8 = 4;
/// Coded Slice of an IDR Partition
pub const IDR_PARTITION: u8 = 5;
/// Supplemental Enhancement Information
pub const SEI: u8 = 6;
/// Sequence Parameter Set
pub const SPS: u8 = 7;
/// Picture Parameter Set
pub const PPS: u8 = 8;
/// Access Unit Delimiter
pub const AUD: u8 = 9;
/// End of Sequence
pub const EOSEQ: u8 = 10;
/// End of Stream
pub const EOS: u8 = 11;
/// Filler Data
pub const FILLER_DATA: u8 = 12;
/// Sequence Parameter Set Extension
pub const SPS_EXTENSION: u8 = 13;
/// Prefix NAL Unit
pub const PREFIX_NAL_UNIT: u8 = 14;
/// Subset Sequence Parameter Set
pub const SUBSET_SPS: u8 = 15;
/// Coded Slice of an Auxiliary Coded Picture Without Partitioning
pub const AUX_PICTURE_WITHOUT_PARTITIONING_SLICE: u8 = 19;
/// Coded Slice Extension
pub const CODED_SLICE_EXTENSION: u8 = 20;
/// Coded Slice Extension for Depth View Components
pub const CODED_SLICE_EXTENSION_FOR_DEPTH_VIEW_COMPONENTS: u8 = 21;
/// STAP-A
pub const STAP_A: u8 = 24;
/// STAP-B
pub const STAP_B: u8 = 25;
/// MTAP16
pub const MTAP_16: u8 = 26;
/// MTAP24
pub const MTAP_24: u8 = 27;
/// FU-A
pub const FU_A: u8 = 28;
/// FU-B
pub const FU_B: u8 = 29;
