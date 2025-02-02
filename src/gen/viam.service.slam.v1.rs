// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// Name of slam service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    /// Current position of the robot within the World frame.
    #[prost(message, optional, tag="1")]
    pub pose: ::core::option::Option<super::super::super::common::v1::PoseInFrame>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMapRequest {
    /// Name of slam service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response (image/jpeg or image/pcd)
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
    /// Optional parameter for image/jpeg mime_type, used to project point
    /// cloud into a 2D image.
    #[prost(message, optional, tag="3")]
    pub camera_position: ::core::option::Option<super::super::super::common::v1::Pose>,
    /// Optional parameter for image/jpeg mime_type, defaults to false.
    /// Tells us whether to include the robot position on the 2D image.
    #[prost(bool, tag="4")]
    pub include_robot_marker: bool,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMapResponse {
    /// Actual MIME type of response (image/jpeg or image/pcd)
    #[prost(string, tag="3")]
    pub mime_type: ::prost::alloc::string::String,
    /// Image or point cloud of mime_type.
    #[prost(oneof="get_map_response::Map", tags="1, 2")]
    pub map: ::core::option::Option<get_map_response::Map>,
}
/// Nested message and enum types in `GetMapResponse`.
pub mod get_map_response {
    /// Image or point cloud of mime_type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Map {
        #[prost(message, tag="1")]
        PointCloud(super::super::super::super::common::v1::PointCloudObject),
        #[prost(bytes, tag="2")]
        Image(::prost::alloc::vec::Vec<u8>),
    }
}
/// Encoded file descriptor set for the `viam.service.slam.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x88, 0x18, 0x0a, 0x1a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x73, 0x6c, 0x61,
    0x6d, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x6c, 0x61, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x14, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x73, 0x6c,
    0x61, 0x6d, 0x2e, 0x76, 0x31, 0x1a, 0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31,
    0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x57, 0x0a, 0x12, 0x47, 0x65, 0x74,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74,
    0x72, 0x61, 0x22, 0x75, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x04, 0x70, 0x6f, 0x73,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x49, 0x6e, 0x46,
    0x72, 0x61, 0x6d, 0x65, 0x52, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78,
    0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0xf9, 0x01, 0x0a, 0x0d, 0x47, 0x65,
    0x74, 0x4d, 0x61, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x1b, 0x0a, 0x09, 0x6d, 0x69, 0x6d, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x08, 0x6d, 0x69, 0x6d, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x42, 0x0a, 0x0f,
    0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x48, 0x00, 0x52, 0x0e, 0x63,
    0x61, 0x6d, 0x65, 0x72, 0x61, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01,
    0x12, 0x30, 0x0a, 0x14, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x5f, 0x72, 0x6f, 0x62, 0x6f,
    0x74, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12,
    0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x52, 0x6f, 0x62, 0x6f, 0x74, 0x4d, 0x61, 0x72, 0x6b,
    0x65, 0x72, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72,
    0x61, 0x42, 0x12, 0x0a, 0x10, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x91, 0x01, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x70,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x43, 0x0a, 0x0b, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x5f, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e,
    0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50,
    0x6f, 0x69, 0x6e, 0x74, 0x43, 0x6c, 0x6f, 0x75, 0x64, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x48,
    0x00, 0x52, 0x0a, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x43, 0x6c, 0x6f, 0x75, 0x64, 0x12, 0x16, 0x0a,
    0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x05,
    0x69, 0x6d, 0x61, 0x67, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x69, 0x6d, 0x65, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x69, 0x6d, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x61, 0x70, 0x32, 0xa9, 0x02, 0x0a, 0x0b, 0x53, 0x4c,
    0x41, 0x4d, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x95, 0x01, 0x0a, 0x0b, 0x47, 0x65,
    0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x73, 0x6c, 0x61, 0x6d, 0x2e, 0x76, 0x31,
    0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x1a, 0x29, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x73, 0x6c, 0x61, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31,
    0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2b, 0x12, 0x29, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70,
    0x69, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x73, 0x6c, 0x61,
    0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x81, 0x01, 0x0a, 0x06, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x12, 0x23, 0x2e, 0x76,
    0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x73, 0x6c, 0x61, 0x6d,
    0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x1a, 0x24, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x73, 0x6c, 0x61, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x2c, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x26, 0x12,
    0x24, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x73, 0x6c, 0x61, 0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65,
    0x7d, 0x2f, 0x6d, 0x61, 0x70, 0x42, 0x3b, 0x0a, 0x18, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x69, 0x61,
    0x6d, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x73, 0x6c, 0x61, 0x6d, 0x2e, 0x76,
    0x31, 0x5a, 0x1f, 0x67, 0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61,
    0x70, 0x69, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x73, 0x6c, 0x61, 0x6d, 0x2f,
    0x76, 0x31, 0x4a, 0xae, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x43, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x1d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06,
    0x00, 0x26, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x08, 0x00, 0x36, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00,
    0x31, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x31, 0x0a, 0x49, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x1c, 0x01, 0x1a, 0x3d, 0x20, 0x41, 0x20, 0x53, 0x6c, 0x61,
    0x6d, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x61, 0x6d, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x13, 0x0a, 0x79, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0f, 0x02, 0x13,
    0x03, 0x1a, 0x6b, 0x20, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x65, 0x73, 0x74, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x64, 0x20, 0x70, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f,
    0x62, 0x6f, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x22, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x22, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x70, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x2f, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x10, 0x04, 0x12, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04,
    0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x10, 0x04, 0x12, 0x06, 0x0a, 0x61, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x01, 0x12, 0x04, 0x17, 0x02, 0x1b, 0x03, 0x1a, 0x53, 0x20, 0x47, 0x65, 0x74, 0x4d, 0x61,
    0x70, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61,
    0x74, 0x65, 0x73, 0x74, 0x20, 0x6d, 0x61, 0x70, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x6f,
    0x72, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x20, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x53, 0x4c, 0x41, 0x4d, 0x20, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x06, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x17, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x17, 0x25, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x18, 0x04, 0x1a, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x01, 0x04, 0xb0,
    0xca, 0xbc, 0x22, 0x12, 0x04, 0x18, 0x04, 0x1a, 0x06, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x1e, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x08,
    0x1a, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x12, 0x1a, 0x16,
    0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x6c, 0x61, 0x6d, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x10, 0x11,
    0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x24, 0x1a, 0x24, 0x20,
    0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68,
    0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x22, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x19, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x21, 0x23, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x25, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x25, 0x08, 0x1b, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x27,
    0x02, 0x21, 0x1a, 0x37, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x62,
    0x6f, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x57, 0x6f,
    0x72, 0x6c, 0x64, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x27, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x27, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x27, 0x1f, 0x20, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x29,
    0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x29, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x29, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29,
    0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2c, 0x00, 0x39, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x15, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x12, 0x1a, 0x16, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x73, 0x6c, 0x61, 0x6d, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x10, 0x11, 0x0a, 0x48, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x30, 0x02, 0x17, 0x1a, 0x3b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x65, 0x64, 0x20, 0x4d, 0x49, 0x4d, 0x45, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x28, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2f,
    0x6a, 0x70, 0x65, 0x67, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2f, 0x70, 0x63,
    0x64, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x30, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x09, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x15, 0x16, 0x0a, 0x69, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x33, 0x02, 0x2e, 0x1a, 0x5c, 0x20, 0x4f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2f, 0x6a, 0x70, 0x65, 0x67, 0x20, 0x6d,
    0x69, 0x6d, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x0a,
    0x20, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x32, 0x44,
    0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x33, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33,
    0x1a, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x2c, 0x2d,
    0x0a, 0x90, 0x01, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x02, 0x20, 0x1a, 0x82,
    0x01, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x2f, 0x6a,
    0x70, 0x65, 0x67, 0x20, 0x6d, 0x69, 0x6d, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x64,
    0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65,
    0x2e, 0x0a, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x75, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x32, 0x44, 0x20, 0x69, 0x6d, 0x61, 0x67,
    0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x36, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x36, 0x07, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x36, 0x1e, 0x1f, 0x0a, 0x31, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x38, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x38, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x38, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x38, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x3b, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3b,
    0x08, 0x16, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x3d, 0x02, 0x40, 0x03,
    0x1a, 0x24, 0x20, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x20, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x69, 0x6d, 0x65, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12,
    0x03, 0x3d, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x04,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3e, 0x04, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x1f, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x3f, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x3f, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3f,
    0x12, 0x13, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x42, 0x02, 0x17, 0x1a,
    0x38, 0x20, 0x41, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x4d, 0x49, 0x4d, 0x45, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x28,
    0x69, 0x6d, 0x61, 0x67, 0x65, 0x2f, 0x6a, 0x70, 0x65, 0x67, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d,
    0x61, 0x67, 0x65, 0x2f, 0x70, 0x63, 0x64, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x42, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x42, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x42, 0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.service.slam.v1.tonic.rs");
// @@protoc_insertion_point(module)