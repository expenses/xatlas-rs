/* automatically generated by rust-bindgen */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub const _SAL_VERSION: u32 = 20;
    pub const __SAL_H_VERSION: u32 = 180000000;
    pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
    pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
    pub const _CRT_PACKING: u32 = 8;
    pub const _HAS_EXCEPTIONS: u32 = 1;
    pub const NULL: u32 = 0;
    pub const WCHAR_MIN: u32 = 0;
    pub const WCHAR_MAX: u32 = 65535;
    pub const WINT_MIN: u32 = 0;
    pub const WINT_MAX: u32 = 65535;
    pub type va_list = *mut ::std::os::raw::c_char;
    extern "C" {
        pub fn __va_start(arg1: *mut root::va_list, ...);
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __vcrt_va_list_is_reference {
        pub _address: u8,
    }
    pub const __vcrt_va_list_is_reference___the_value:
        root::__vcrt_va_list_is_reference__bindgen_ty_1 = 0;
    pub type __vcrt_va_list_is_reference__bindgen_ty_1 = u8;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct __vcrt_assert_va_start_is_not_reference {
        pub _address: u8,
    }
    pub type __vcrt_bool = bool;
    extern "C" {
        pub fn __security_init_cookie();
    }
    extern "C" {
        pub fn __security_check_cookie(_StackCookie: usize);
    }
    extern "C" {
        pub fn __report_gsfailure(_StackCookie: usize);
    }
    extern "C" {
        pub static mut __security_cookie: usize;
    }
    pub type int_least8_t = ::std::os::raw::c_schar;
    pub type int_least16_t = ::std::os::raw::c_short;
    pub type int_least32_t = ::std::os::raw::c_int;
    pub type int_least64_t = ::std::os::raw::c_longlong;
    pub type uint_least8_t = ::std::os::raw::c_uchar;
    pub type uint_least16_t = ::std::os::raw::c_ushort;
    pub type uint_least32_t = ::std::os::raw::c_uint;
    pub type uint_least64_t = ::std::os::raw::c_ulonglong;
    pub type int_fast8_t = ::std::os::raw::c_schar;
    pub type int_fast16_t = ::std::os::raw::c_int;
    pub type int_fast32_t = ::std::os::raw::c_int;
    pub type int_fast64_t = ::std::os::raw::c_longlong;
    pub type uint_fast8_t = ::std::os::raw::c_uchar;
    pub type uint_fast16_t = ::std::os::raw::c_uint;
    pub type uint_fast32_t = ::std::os::raw::c_uint;
    pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
    pub type intmax_t = ::std::os::raw::c_longlong;
    pub type uintmax_t = ::std::os::raw::c_ulonglong;
    pub mod xatlas {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Chart {
            pub atlasIndex: u32,
            pub indexArray: *mut u32,
            pub indexCount: u32,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Vertex {
            pub atlasIndex: i32,
            pub uv: [f32; 2usize],
            pub xref: u32,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Mesh {
            pub chartArray: *mut root::xatlas::Chart,
            pub chartCount: u32,
            pub indexArray: *mut u32,
            pub indexCount: u32,
            pub vertexArray: *mut root::xatlas::Vertex,
            pub vertexCount: u32,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Atlas {
            pub width: u32,
            pub height: u32,
            pub atlasCount: u32,
            pub chartCount: u32,
            pub meshCount: u32,
            pub meshes: *mut root::xatlas::Mesh,
            pub utilization: *mut f32,
            pub texelsPerUnit: f32,
        }
        extern "C" {
            #[link_name = "\u{1}?Create@xatlas@@YAPEAUAtlas@1@XZ"]
            pub fn Create() -> *mut root::xatlas::Atlas;
        }
        extern "C" {
            #[link_name = "\u{1}?Destroy@xatlas@@YAXPEAUAtlas@1@@Z"]
            pub fn Destroy(atlas: *mut root::xatlas::Atlas);
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct IndexFormat {
            pub _address: u8,
        }
        pub const IndexFormat_Enum_UInt16: root::xatlas::IndexFormat_Enum = 0;
        pub const IndexFormat_Enum_UInt32: root::xatlas::IndexFormat_Enum = 1;
        pub type IndexFormat_Enum = i32;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct MeshDecl {
            pub vertexCount: u32,
            pub vertexPositionData: *const ::std::os::raw::c_void,
            pub vertexPositionStride: u32,
            pub vertexNormalData: *const ::std::os::raw::c_void,
            pub vertexNormalStride: u32,
            pub vertexUvData: *const ::std::os::raw::c_void,
            pub vertexUvStride: u32,
            pub indexCount: u32,
            pub indexData: *const ::std::os::raw::c_void,
            pub indexOffset: i32,
            pub indexFormat: root::xatlas::IndexFormat_Enum,
            pub faceIgnoreData: *const bool,
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct AddMeshError {
            pub _address: u8,
        }
        pub const AddMeshError_Enum_Success: root::xatlas::AddMeshError_Enum = 0;
        pub const AddMeshError_Enum_IndexOutOfRange: root::xatlas::AddMeshError_Enum = 1;
        pub const AddMeshError_Enum_InvalidIndexCount: root::xatlas::AddMeshError_Enum = 2;
        pub type AddMeshError_Enum = i32;
        extern "C" {
            #[link_name = "\u{1}?AddMesh@xatlas@@YA?AW4Enum@AddMeshError@1@PEAUAtlas@1@AEBUMeshDecl@1@@Z"]
            pub fn AddMesh(
                atlas: *mut root::xatlas::Atlas,
                meshDecl: *const root::xatlas::MeshDecl,
            ) -> root::xatlas::AddMeshError_Enum;
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ProgressCategory {
            pub _address: u8,
        }
        pub const ProgressCategory_Enum_ComputeCharts: root::xatlas::ProgressCategory_Enum = 0;
        pub const ProgressCategory_Enum_ParameterizeCharts: root::xatlas::ProgressCategory_Enum = 1;
        pub const ProgressCategory_Enum_PackCharts: root::xatlas::ProgressCategory_Enum = 2;
        pub const ProgressCategory_Enum_BuildOutputMeshes: root::xatlas::ProgressCategory_Enum = 3;
        pub type ProgressCategory_Enum = i32;
        pub type ProgressFunc = ::std::option::Option<
            unsafe extern "C" fn(
                category: root::xatlas::ProgressCategory_Enum,
                progress: ::std::os::raw::c_int,
                userData: *mut ::std::os::raw::c_void,
            ),
        >;
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct ChartOptions {
            pub maxChartArea: f32,
            pub maxBoundaryLength: f32,
            pub proxyFitMetricWeight: f32,
            pub roundnessMetricWeight: f32,
            pub straightnessMetricWeight: f32,
            pub normalSeamMetricWeight: f32,
            pub textureSeamMetricWeight: f32,
            pub maxThreshold: f32,
            pub growFaceCount: u32,
            pub maxIterations: u32,
        }
        extern "C" {
            #[link_name = "\u{1}?ComputeCharts@xatlas@@YAXPEAUAtlas@1@UChartOptions@1@P6AXW4Enum@ProgressCategory@1@HPEAX@Z3@Z"]
            pub fn ComputeCharts(
                atlas: *mut root::xatlas::Atlas,
                chartOptions: root::xatlas::ChartOptions,
                progressFunc: root::xatlas::ProgressFunc,
                progressUserData: *mut ::std::os::raw::c_void,
            );
        }
        pub type ParameterizeFunc = ::std::option::Option<
            unsafe extern "C" fn(
                positions: *const f32,
                texcoords: *mut f32,
                vertexCount: u32,
                indices: *const u32,
                indexCount: u32,
                isPlanar: bool,
            ),
        >;
        extern "C" {
            #[link_name = "\u{1}?ParameterizeCharts@xatlas@@YAXPEAUAtlas@1@P6AXPEBMPEAMIPEBII_N@ZP6AXW4Enum@ProgressCategory@1@HPEAX@Z7@Z"]
            pub fn ParameterizeCharts(
                atlas: *mut root::xatlas::Atlas,
                func: root::xatlas::ParameterizeFunc,
                progressFunc: root::xatlas::ProgressFunc,
                progressUserData: *mut ::std::os::raw::c_void,
            );
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct PackOptions {
            pub attempts: ::std::os::raw::c_int,
            pub texelsPerUnit: f32,
            pub resolution: u32,
            pub maxChartSize: u32,
            pub blockAlign: bool,
            pub conservative: bool,
            pub padding: u32,
        }
        extern "C" {
            #[link_name = "\u{1}?PackCharts@xatlas@@YAXPEAUAtlas@1@UPackOptions@1@P6AXW4Enum@ProgressCategory@1@HPEAX@Z3@Z"]
            pub fn PackCharts(
                atlas: *mut root::xatlas::Atlas,
                packOptions: root::xatlas::PackOptions,
                progressFunc: root::xatlas::ProgressFunc,
                progressUserData: *mut ::std::os::raw::c_void,
            );
        }
        extern "C" {
            #[link_name = "\u{1}?Generate@xatlas@@YAXPEAUAtlas@1@UChartOptions@1@P6AXPEBMPEAMIPEBII_N@ZUPackOptions@1@P6AXW4Enum@ProgressCategory@1@HPEAX@Z9@Z"]
            pub fn Generate(
                atlas: *mut root::xatlas::Atlas,
                chartOptions: root::xatlas::ChartOptions,
                paramFunc: root::xatlas::ParameterizeFunc,
                packOptions: root::xatlas::PackOptions,
                progressFunc: root::xatlas::ProgressFunc,
                progressUserData: *mut ::std::os::raw::c_void,
            );
        }
        pub type ReallocFunc = ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: usize,
            ) -> *mut ::std::os::raw::c_void,
        >;
        extern "C" {
            #[link_name = "\u{1}?SetRealloc@xatlas@@YAXP6APEAXPEAX_K@Z@Z"]
            pub fn SetRealloc(reallocFunc: root::xatlas::ReallocFunc);
        }
        pub type PrintFunc = ::std::option::Option<
            unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int,
        >;
        extern "C" {
            #[link_name = "\u{1}?SetPrint@xatlas@@YAXP6AHPEBDZZ_N@Z"]
            pub fn SetPrint(print: root::xatlas::PrintFunc, verbose: bool);
        }
        extern "C" {
            #[link_name = "\u{1}?StringForEnum@xatlas@@YAPEBDW4Enum@AddMeshError@1@@Z"]
            pub fn StringForEnum(
                error: root::xatlas::AddMeshError_Enum,
            ) -> *const ::std::os::raw::c_char;
        }
        extern "C" {
            #[link_name = "\u{1}?StringForEnum@xatlas@@YAPEBDW4Enum@ProgressCategory@1@@Z"]
            pub fn StringForEnum1(
                category: root::xatlas::ProgressCategory_Enum,
            ) -> *const ::std::os::raw::c_char;
        }
    }
}
