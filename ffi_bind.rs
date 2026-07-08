#![allow(warnings)]
use std::ffi::{c_char, c_uchar, c_void};
use std::ffi::CString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type Quaternion = Vector4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: u32,
    pub texture: Texture,
    pub depth: Texture,
}
pub type RenderTexture2D = RenderTexture;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub layout: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfo {
    pub value: i32,
    pub offsetX: i32,
    pub offsetY: i32,
    pub advanceX: i32,
    pub image: Image,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub baseSize: i32,
    pub glyphCount: i32,
    pub glyphPadding: i32,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: i32,
}
pub type Camera = Camera3D;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertexCount: i32,
    pub triangleCount: i32,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub texcoords2: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut c_uchar,
    pub indices: *mut u16,
    pub boneCount: i32,
    pub boneIndices: *mut c_uchar,
    pub boneWeights: *mut f32,
    pub animVertices: *mut f32,
    pub animNormals: *mut f32,
    pub vaoId: u32,
    pub vboId: *mut u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub id: u32,
    pub locs: *mut i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
    pub texture: Texture2D,
    pub color: Color,
    pub value: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub shader: Shader,
    pub maps: *mut MaterialMap,
    pub params: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
pub type ModelAnimPose = *mut Transform;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoneInfo {
    pub name: [i8; 32],
    pub parent: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelSkeleton {
    pub boneCount: i32,
    pub bones: *mut BoneInfo,
    pub bindPose: ModelAnimPose,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
    pub transform: Matrix,
    pub meshCount: i32,
    pub materialCount: i32,
    pub meshes: *mut Mesh,
    pub materials: *mut Material,
    pub meshMaterial: *mut i32,
    pub skeleton: ModelSkeleton,
    pub currentPose: ModelAnimPose,
    pub boneMatrices: *mut Matrix,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelAnimation {
    pub name: [i8; 32],
    pub boneCount: i32,
    pub keyframeCount: i32,
    pub keyframePoses: *mut ModelAnimPose,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayCollision {
    pub hit: bool,
    pub distance: f32,
    pub point: Vector3,
    pub normal: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub frameCount: u32,
    pub sampleRate: u32,
    pub sampleSize: u32,
    pub channels: u32,
    pub data: *mut c_void,
}
#[repr(C)]
pub struct rAudioBuffer {
     _unused: [u8; 0],
}
#[repr(C)]
pub struct rAudioProcessor {
     _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStream {
    pub buffer: *mut rAudioBuffer,
    pub processor: *mut rAudioProcessor,
    pub sampleRate: u32,
    pub sampleSize: u32,
    pub channels: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
    pub stream: AudioStream,
    pub frameCount: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
    pub stream: AudioStream,
    pub frameCount: u32,
    pub looping: bool,
    pub ctxType: i32,
    pub ctxData: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrDeviceInfo {
    pub hResolution: i32,
    pub vResolution: i32,
    pub hScreenSize: f32,
    pub vScreenSize: f32,
    pub eyeToScreenDistance: f32,
    pub lensSeparationDistance: f32,
    pub interpupillaryDistance: f32,
    pub lensDistortionValues: [f32; 4],
    pub chromaAbCorrection: [f32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2],
    pub viewOffset: [Matrix; 2],
    pub leftLensCenter: [f32; 2],
    pub rightLensCenter: [f32; 2],
    pub leftScreenCenter: [f32; 2],
    pub rightScreenCenter: [f32; 2],
    pub scale: [f32; 2],
    pub scaleIn: [f32; 2],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FilePathList {
    pub count: u32,
    pub paths: *mut c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEvent {
    pub frame: u32,
    pub r#type: u32,
    pub params: [i32; 4],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AutomationEventList {
    pub capacity: u32,
    pub count: u32,
    pub events: *mut AutomationEvent,
}
enum ConfigFlags {
    FLAG_VSYNC_HINT = 0x00000040,
    FLAG_FULLSCREEN_MODE = 0x00000002,
    FLAG_WINDOW_RESIZABLE = 0x00000004,
    FLAG_WINDOW_UNDECORATED = 0x00000008,
    FLAG_WINDOW_HIDDEN = 0x00000080,
    FLAG_WINDOW_MINIMIZED = 0x00000200,
    FLAG_WINDOW_MAXIMIZED = 0x00000400,
    FLAG_WINDOW_UNFOCUSED = 0x00000800,
    FLAG_WINDOW_TOPMOST = 0x00001000,
    FLAG_WINDOW_ALWAYS_RUN = 0x00000100,
    FLAG_WINDOW_TRANSPARENT = 0x00000010,
    FLAG_WINDOW_HIGHDPI = 0x00002000,
    FLAG_WINDOW_MOUSE_PASSTHROUGH = 0x00004000,
    FLAG_BORDERLESS_WINDOWED_MODE = 0x00008000,
    FLAG_MSAA_4X_HINT = 0x00000020,
}
enum TraceLogLevel {
    LOG_ALL = 0,
    LOG_TRACE,
    LOG_DEBUG,
    LOG_INFO,
    LOG_WARNING,
    LOG_ERROR,
    LOG_FATAL,
}
enum KeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_MENU = 5,
    KEY_VOLUME_UP = 24,
}
enum MouseButton {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
}
enum MouseCursor {
    MOUSE_CURSOR_DEFAULT = 0,
    MOUSE_CURSOR_ARROW = 1,
    MOUSE_CURSOR_IBEAM = 2,
    MOUSE_CURSOR_CROSSHAIR = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW = 5,
    MOUSE_CURSOR_RESIZE_NS = 6,
    MOUSE_CURSOR_RESIZE_NWSE = 7,
    MOUSE_CURSOR_RESIZE_NESW = 8,
    MOUSE_CURSOR_RESIZE_ALL = 9,
}
enum GamepadButton {
    GAMEPAD_BUTTON_UNKNOWN = 0,
    GAMEPAD_BUTTON_LEFT_FACE_UP,
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
    GAMEPAD_BUTTON_LEFT_FACE_DOWN,
    GAMEPAD_BUTTON_LEFT_FACE_LEFT,
    GAMEPAD_BUTTON_RIGHT_FACE_UP,
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
    GAMEPAD_BUTTON_LEFT_TRIGGER_1,
    GAMEPAD_BUTTON_LEFT_TRIGGER_2,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
    GAMEPAD_BUTTON_MIDDLE_LEFT,
    GAMEPAD_BUTTON_MIDDLE,
    GAMEPAD_BUTTON_MIDDLE_RIGHT,
    GAMEPAD_BUTTON_LEFT_THUMB,
}
enum GamepadAxis {
    GAMEPAD_AXIS_LEFT_X = 0,
    GAMEPAD_AXIS_LEFT_Y = 1,
    GAMEPAD_AXIS_RIGHT_X = 2,
    GAMEPAD_AXIS_RIGHT_Y = 3,
    GAMEPAD_AXIS_LEFT_TRIGGER = 4,
}
enum MaterialMapIndex {
    MATERIAL_MAP_ALBEDO = 0,
    MATERIAL_MAP_METALNESS,
    MATERIAL_MAP_NORMAL,
    MATERIAL_MAP_ROUGHNESS,
    MATERIAL_MAP_OCCLUSION,
    MATERIAL_MAP_EMISSION,
    MATERIAL_MAP_HEIGHT,
    MATERIAL_MAP_CUBEMAP,
    MATERIAL_MAP_IRRADIANCE,
    MATERIAL_MAP_PREFILTER,
}
enum ShaderLocationIndex {
    SHADER_LOC_VERTEX_POSITION = 0,
    SHADER_LOC_VERTEX_TEXCOORD01,
    SHADER_LOC_VERTEX_TEXCOORD02,
    SHADER_LOC_VERTEX_NORMAL,
    SHADER_LOC_VERTEX_TANGENT,
    SHADER_LOC_VERTEX_COLOR,
    SHADER_LOC_MATRIX_MVP,
    SHADER_LOC_MATRIX_VIEW,
    SHADER_LOC_MATRIX_PROJECTION,
    SHADER_LOC_MATRIX_MODEL,
    SHADER_LOC_MATRIX_NORMAL,
    SHADER_LOC_VECTOR_VIEW,
    SHADER_LOC_COLOR_DIFFUSE,
    SHADER_LOC_COLOR_SPECULAR,
    SHADER_LOC_COLOR_AMBIENT,
    SHADER_LOC_MAP_ALBEDO,
    SHADER_LOC_MAP_METALNESS,
    SHADER_LOC_MAP_NORMAL,
    SHADER_LOC_MAP_ROUGHNESS,
    SHADER_LOC_MAP_OCCLUSION,
    SHADER_LOC_MAP_EMISSION,
    SHADER_LOC_MAP_HEIGHT,
    SHADER_LOC_MAP_CUBEMAP,
    SHADER_LOC_MAP_IRRADIANCE,
    SHADER_LOC_MAP_PREFILTER,
    SHADER_LOC_MAP_BRDF,
    SHADER_LOC_VERTEX_BONEIDS,
    SHADER_LOC_VERTEX_BONEWEIGHTS,
    SHADER_LOC_MATRIX_BONETRANSFORMS,
}
enum ShaderUniformDataType {
    SHADER_UNIFORM_FLOAT = 0,
    SHADER_UNIFORM_VEC2,
    SHADER_UNIFORM_VEC3,
    SHADER_UNIFORM_VEC4,
    SHADER_UNIFORM_INT,
    SHADER_UNIFORM_IVEC2,
    SHADER_UNIFORM_IVEC3,
    SHADER_UNIFORM_IVEC4,
    SHADER_UNIFORM_UINT,
    SHADER_UNIFORM_UIVEC2,
    SHADER_UNIFORM_UIVEC3,
    SHADER_UNIFORM_UIVEC4,
}
enum ShaderAttributeDataType {
    SHADER_ATTRIB_FLOAT = 0,
    SHADER_ATTRIB_VEC2,
    SHADER_ATTRIB_VEC3,
}
enum PixelFormat {
    PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA,
    PIXELFORMAT_UNCOMPRESSED_R5G6B5,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8,
    PIXELFORMAT_UNCOMPRESSED_R5G5B5A1,
    PIXELFORMAT_UNCOMPRESSED_R4G4B4A4,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
    PIXELFORMAT_UNCOMPRESSED_R32,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32A32,
    PIXELFORMAT_UNCOMPRESSED_R16,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16A16,
    PIXELFORMAT_COMPRESSED_DXT1_RGB,
    PIXELFORMAT_COMPRESSED_DXT1_RGBA,
    PIXELFORMAT_COMPRESSED_DXT3_RGBA,
    PIXELFORMAT_COMPRESSED_DXT5_RGBA,
    PIXELFORMAT_COMPRESSED_ETC1_RGB,
    PIXELFORMAT_COMPRESSED_ETC2_RGB,
    PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA,
    PIXELFORMAT_COMPRESSED_PVRT_RGB,
    PIXELFORMAT_COMPRESSED_PVRT_RGBA,
    PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA,
}
enum TextureFilter {
    TEXTURE_FILTER_POINT = 0,
    TEXTURE_FILTER_BILINEAR,
    TEXTURE_FILTER_TRILINEAR,
    TEXTURE_FILTER_ANISOTROPIC_4X,
    TEXTURE_FILTER_ANISOTROPIC_8X,
}
enum TextureWrap {
    TEXTURE_WRAP_REPEAT = 0,
    TEXTURE_WRAP_CLAMP,
    TEXTURE_WRAP_MIRROR_REPEAT,
}
enum CubemapLayout {
    CUBEMAP_LAYOUT_AUTO_DETECT = 0,
    CUBEMAP_LAYOUT_LINE_VERTICAL,
    CUBEMAP_LAYOUT_LINE_HORIZONTAL,
    CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR,
}
enum FontType {
    FONT_DEFAULT = 0,
    FONT_BITMAP,
}
enum BlendMode {
    BLEND_ALPHA = 0,
    BLEND_ADDITIVE,
    BLEND_MULTIPLIED,
    BLEND_ADD_COLORS,
    BLEND_SUBTRACT_COLORS,
    BLEND_ALPHA_PREMULTIPLY,
    BLEND_CUSTOM,
}
enum Gesture {
    GESTURE_NONE = 0,
    GESTURE_TAP = 1,
    GESTURE_DOUBLETAP = 2,
    GESTURE_HOLD = 4,
    GESTURE_DRAG = 8,
    GESTURE_SWIPE_RIGHT = 16,
    GESTURE_SWIPE_LEFT = 32,
    GESTURE_SWIPE_UP = 64,
    GESTURE_SWIPE_DOWN = 128,
    GESTURE_PINCH_IN = 256,
}
enum CameraMode {
    CAMERA_CUSTOM = 0,
    CAMERA_FREE,
    CAMERA_ORBITAL,
    CAMERA_FIRST_PERSON,
}
enum CameraProjection {
    CAMERA_PERSPECTIVE = 0,
}
enum NPatchLayout {
    NPATCH_NINE_PATCH = 0,
    NPATCH_THREE_PATCH_VERTICAL,
}
mod raw{
    use ffi_bind::*;
    unsafe extern "C" {
        pub fn InitWindow(width: i32, height: i32, title: *const c_char);
        pub fn CloseWindow();
        pub fn WindowShouldClose()->bool;
        pub fn IsWindowReady()->bool;
        pub fn IsWindowFullscreen()->bool;
        pub fn IsWindowHidden()->bool;
        pub fn IsWindowMinimized()->bool;
        pub fn IsWindowMaximized()->bool;
        pub fn IsWindowFocused()->bool;
        pub fn IsWindowResized()->bool;
        pub fn IsWindowState(flag: u32)->bool;
        pub fn SetWindowState(flags: u32);
        pub fn ClearWindowState(flags: u32);
        pub fn ToggleFullscreen();
        pub fn ToggleBorderlessWindowed();
        pub fn MaximizeWindow();
        pub fn MinimizeWindow();
        pub fn RestoreWindow();
        pub fn SetWindowIcon(image: Image);
        pub fn SetWindowIcons(images: *mut Image, count: i32);
        pub fn SetWindowTitle(title: *const c_char);
        pub fn SetWindowPosition(x: i32, y: i32);
        pub fn SetWindowMonitor(monitor: i32);
        pub fn SetWindowMinSize(width: i32, height: i32);
        pub fn SetWindowMaxSize(width: i32, height: i32);
        pub fn SetWindowSize(width: i32, height: i32);
        pub fn SetWindowOpacity(opacity: f32);
        pub fn SetWindowFocused();
        pub fn GetWindowHandle()->*mut c_void;
        pub fn GetScreenWidth()->i32;
        pub fn GetScreenHeight()->i32;
        pub fn GetRenderWidth()->i32;
        pub fn GetRenderHeight()->i32;
        pub fn GetMonitorCount()->i32;
        pub fn GetCurrentMonitor()->i32;
        pub fn GetMonitorPosition(monitor: i32)->Vector2;
        pub fn GetMonitorWidth(monitor: i32)->i32;
        pub fn GetMonitorHeight(monitor: i32)->i32;
        pub fn GetMonitorPhysicalWidth(monitor: i32)->i32;
        pub fn GetMonitorPhysicalHeight(monitor: i32)->i32;
        pub fn GetMonitorRefreshRate(monitor: i32)->i32;
        pub fn GetWindowPosition()->Vector2;
        pub fn GetWindowScaleDPI()->Vector2;
        pub fn GetMonitorName(monitor: i32)->*const c_char;
        pub fn SetClipboardText(text: *const c_char);
        pub fn GetClipboardText()->*const c_char;
        pub fn GetClipboardImage()->Image;
        pub fn EnableEventWaiting();
        pub fn DisableEventWaiting();
        pub fn ShowCursor();
        pub fn HideCursor();
        pub fn IsCursorHidden()->bool;
        pub fn EnableCursor();
        pub fn DisableCursor();
        pub fn IsCursorOnScreen()->bool;
        pub fn ClearBackground(color: Color);
        pub fn BeginDrawing();
        pub fn EndDrawing();
        pub fn BeginMode2D(camera: Camera2D);
        pub fn EndMode2D();
        pub fn BeginMode3D(camera: Camera3D);
        pub fn EndMode3D();
        pub fn BeginTextureMode(target: RenderTexture2D);
        pub fn EndTextureMode();
        pub fn BeginShaderMode(shader: Shader);
        pub fn EndShaderMode();
        pub fn BeginBlendMode(mode: i32);
        pub fn EndBlendMode();
        pub fn BeginScissorMode(x: i32, y: i32, width: i32, height: i32);
        pub fn EndScissorMode();
        pub fn BeginVrStereoMode(config: VrStereoConfig);
        pub fn EndVrStereoMode();
        pub fn LoadVrStereoConfig(device: VrDeviceInfo)->VrStereoConfig;
        pub fn UnloadVrStereoConfig(config: VrStereoConfig);
        pub fn LoadShader(vsFileName: *const c_char, fsFileName: *const c_char)->Shader;
        pub fn LoadShaderFromMemory(vsCode: *const c_char, fsCode: *const c_char)->Shader;
        pub fn IsShaderValid(shader: Shader)->bool;
        pub fn GetShaderLocation(shader: Shader, uniformName: *const c_char)->i32;
        pub fn GetShaderLocationAttrib(shader: Shader, attribName: *const c_char)->i32;
        pub fn SetShaderValue(shader: Shader, locIndex: i32, value: *const c_void, uniformType: i32);
        pub fn SetShaderValueV(shader: Shader, locIndex: i32, value: *const c_void, uniformType: i32, count: i32);
        pub fn SetShaderValueMatrix(shader: Shader, locIndex: i32, mat: Matrix);
        pub fn SetShaderValueTexture(shader: Shader, locIndex: i32, texture: Texture2D);
        pub fn UnloadShader(shader: Shader);
        pub fn GetScreenToWorldRay(position: Vector2, camera: Camera)->Ray;
        pub fn GetScreenToWorldRayEx(position: Vector2, camera: Camera, width: i32, height: i32)->Ray;
        pub fn GetWorldToScreen(position: Vector3, camera: Camera)->Vector2;
        pub fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: i32, height: i32)->Vector2;
        pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D)->Vector2;
        pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D)->Vector2;
        pub fn GetCameraMatrix(camera: Camera)->Matrix;
        pub fn GetCameraMatrix2D(camera: Camera2D)->Matrix;
        pub fn SetTargetFPS(fps: i32);
        pub fn GetFrameTime()->f32;
        pub fn GetTime()->f64;
        pub fn GetFPS()->i32;
        pub fn SwapScreenBuffer();
        pub fn PollInputEvents();
        pub fn WaitTime(seconds: f64);
        pub fn SetRandomSeed(seed: u32);
        pub fn GetRandomValue(min: i32, max: i32)->i32;
        pub fn LoadRandomSequence(count: u32, min: i32, max: i32)->*mut i32;
        pub fn UnloadRandomSequence(sequence: *mut i32);
        pub fn TakeScreenshot(fileName: *const c_char);
        pub fn SetConfigFlags(flags: u32);
        pub fn OpenURL(url: *const c_char);
        pub fn SetTraceLogLevel(logLevel: i32);
        pub fn TraceLog(logLevel: i32, text: *const c_char, ...);
        pub fn MemAlloc(size: u32)->*mut c_void;
        pub fn MemRealloc(ptr: *mut c_void, size: u32)->*mut c_void;
        pub fn MemFree(ptr: *mut c_void);
        pub fn LoadFileData(fileName: *const c_char, dataSize: *mut i32)->*mut c_uchar;
        pub fn UnloadFileData(data: *mut c_uchar);
        pub fn SaveFileData(fileName: *const c_char, data: *const c_void, dataSize: i32)->bool;
        pub fn ExportDataAsCode(data: *const c_uchar, dataSize: i32, fileName: *const c_char)->bool;
        pub fn LoadFileText(fileName: *const c_char)->*mut c_char;
        pub fn UnloadFileText(text: *mut c_char);
        pub fn SaveFileText(fileName: *const c_char, text: *const c_char)->bool;
        pub fn FileRename(fileName: *const c_char, fileRename: *const c_char)->i32;
        pub fn FileRemove(fileName: *const c_char)->i32;
        pub fn FileCopy(srcPath: *const c_char, dstPath: *const c_char)->i32;
        pub fn FileMove(srcPath: *const c_char, dstPath: *const c_char)->i32;
        pub fn FileTextReplace(fileName: *const c_char, search: *const c_char, replacement: *const c_char)->i32;
        pub fn FileTextFindIndex(fileName: *const c_char, search: *const c_char)->i32;
        pub fn FileExists(fileName: *const c_char)->bool;
        pub fn DirectoryExists(dirPath: *const c_char)->bool;
        pub fn IsFileExtension(fileName: *const c_char, ext: *const c_char)->bool;
        pub fn GetFileLength(fileName: *const c_char)->i32;
        pub fn GetFileModTime(fileName: *const c_char)->i64;
        pub fn GetFileExtension(fileName: *const c_char)->*const c_char;
        pub fn GetFileName(filePath: *const c_char)->*const c_char;
        pub fn GetFileNameWithoutExt(filePath: *const c_char)->*const c_char;
        pub fn GetDirectoryPath(filePath: *const c_char)->*const c_char;
        pub fn GetPrevDirectoryPath(dirPath: *const c_char)->*const c_char;
        pub fn GetWorkingDirectory()->*const c_char;
        pub fn GetApplicationDirectory()->*const c_char;
        pub fn MakeDirectory(dirPath: *const c_char)->i32;
        pub fn ChangeDirectory(dirPath: *const c_char)->i32;
        pub fn IsPathFile(path: *const c_char)->bool;
        pub fn IsPathDirectory(path: *const c_char)->bool;
        pub fn IsFileNameValid(fileName: *const c_char)->bool;
        pub fn LoadDirectoryFiles(dirPath: *const c_char)->FilePathList;
        pub fn LoadDirectoryFilesEx(basePath: *const c_char, filter: *const c_char, scanSubdirs: bool)->FilePathList;
        pub fn UnloadDirectoryFiles(files: FilePathList);
        pub fn IsFileDropped()->bool;
        pub fn LoadDroppedFiles()->FilePathList;
        pub fn UnloadDroppedFiles(files: FilePathList);
        pub fn GetDirectoryFileCount(dirPath: *const c_char)->u32;
        pub fn GetDirectoryFileCountEx(basePath: *const c_char, filter: *const c_char, scanSubdirs: bool)->u32;
        pub fn CompressData(data: *const c_uchar, dataSize: i32, compDataSize: *mut i32)->*mut c_uchar;
        pub fn DecompressData(compData: *const c_uchar, compDataSize: i32, dataSize: *mut i32)->*mut c_uchar;
        pub fn EncodeDataBase64(data: *const c_uchar, dataSize: i32, outputSize: *mut i32)->*mut c_char;
        pub fn DecodeDataBase64(text: *const c_char, outputSize: *mut i32)->*mut c_uchar;
        pub fn ComputeCRC32(data: *const c_uchar, dataSize: i32)->u32;
        pub fn ComputeMD5(data: *const c_uchar, dataSize: i32)->*mut u32;
        pub fn ComputeSHA1(data: *const c_uchar, dataSize: i32)->*mut u32;
        pub fn ComputeSHA256(data: *const c_uchar, dataSize: i32)->*mut u32;
        pub fn LoadAutomationEventList(fileName: *const c_char)->AutomationEventList;
        pub fn UnloadAutomationEventList(list: AutomationEventList);
        pub fn ExportAutomationEventList(list: AutomationEventList, fileName: *const c_char)->bool;
        pub fn SetAutomationEventList(list: *mut AutomationEventList);
        pub fn SetAutomationEventBaseFrame(frame: i32);
        pub fn StartAutomationEventRecording();
        pub fn StopAutomationEventRecording();
        pub fn PlayAutomationEvent(event: AutomationEvent);
        pub fn IsKeyPressed(key: i32)->bool;
        pub fn IsKeyPressedRepeat(key: i32)->bool;
        pub fn IsKeyDown(key: i32)->bool;
        pub fn IsKeyReleased(key: i32)->bool;
        pub fn IsKeyUp(key: i32)->bool;
        pub fn GetKeyPressed()->i32;
        pub fn GetCharPressed()->i32;
        pub fn GetKeyName(key: i32)->*const c_char;
        pub fn SetExitKey(key: i32);
        pub fn IsGamepadAvailable(gamepad: i32)->bool;
        pub fn GetGamepadName(gamepad: i32)->*const c_char;
        pub fn IsGamepadButtonPressed(gamepad: i32, button: i32)->bool;
        pub fn IsGamepadButtonDown(gamepad: i32, button: i32)->bool;
        pub fn IsGamepadButtonReleased(gamepad: i32, button: i32)->bool;
        pub fn IsGamepadButtonUp(gamepad: i32, button: i32)->bool;
        pub fn GetGamepadButtonPressed()->i32;
        pub fn GetGamepadAxisCount(gamepad: i32)->i32;
        pub fn GetGamepadAxisMovement(gamepad: i32, axis: i32)->f32;
        pub fn SetGamepadMappings(mappings: *const c_char)->i32;
        pub fn SetGamepadVibration(gamepad: i32, leftMotor: f32, rightMotor: f32, duration: f32);
        pub fn IsMouseButtonPressed(button: i32)->bool;
        pub fn IsMouseButtonDown(button: i32)->bool;
        pub fn IsMouseButtonReleased(button: i32)->bool;
        pub fn IsMouseButtonUp(button: i32)->bool;
        pub fn GetMouseX()->i32;
        pub fn GetMouseY()->i32;
        pub fn GetMousePosition()->Vector2;
        pub fn GetMouseDelta()->Vector2;
        pub fn SetMousePosition(x: i32, y: i32);
        pub fn SetMouseOffset(offsetX: i32, offsetY: i32);
        pub fn SetMouseScale(scaleX: f32, scaleY: f32);
        pub fn GetMouseWheelMove()->f32;
        pub fn GetMouseWheelMoveV()->Vector2;
        pub fn SetMouseCursor(cursor: i32);
        pub fn GetTouchX()->i32;
        pub fn GetTouchY()->i32;
        pub fn GetTouchPosition(index: i32)->Vector2;
        pub fn GetTouchPointId(index: i32)->i32;
        pub fn GetTouchPointCount()->i32;
        pub fn SetGesturesEnabled(flags: u32);
        pub fn IsGestureDetected(gesture: u32)->bool;
        pub fn GetGestureDetected()->i32;
        pub fn GetGestureHoldDuration()->f32;
        pub fn GetGestureDragVector()->Vector2;
        pub fn GetGestureDragAngle()->f32;
        pub fn GetGesturePinchVector()->Vector2;
        pub fn GetGesturePinchAngle()->f32;
        pub fn UpdateCamera(camera: *mut Camera, mode: i32);
        pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: f32);
        pub fn SetShapesTexture(texture: Texture2D, rec: Rectangle);
        pub fn GetShapesTexture()->Texture2D;
        pub fn GetShapesTextureRectangle()->Rectangle;
        pub fn DrawPixel(posX: i32, posY: i32, color: Color);
        pub fn DrawPixelV(position: Vector2, color: Color);
        pub fn DrawLine(startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color);
        pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
        pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
        pub fn DrawLineStrip(points: *const Vector2, pointCount: i32, color: Color);
        pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);
        pub fn DrawLineDashed(startPos: Vector2, endPos: Vector2, dashSize: i32, spaceSize: i32, color: Color);
        pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
        pub fn DrawTriangleGradient(v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color);
        pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
        pub fn DrawTriangleFan(points: *const Vector2, pointCount: i32, color: Color);
        pub fn DrawTriangleStrip(points: *const Vector2, pointCount: i32, color: Color);
        pub fn DrawRectangle(posX: i32, posY: i32, width: i32, height: i32, color: Color);
        pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
        pub fn DrawRectangleRec(rec: Rectangle, color: Color);
        pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
        pub fn DrawRectangleGradientV(posX: i32, posY: i32, width: i32, height: i32, top: Color, bottom: Color);
        pub fn DrawRectangleGradientH(posX: i32, posY: i32, width: i32, height: i32, left: Color, right: Color);
        pub fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color);
        pub fn DrawRectangleLines(posX: i32, posY: i32, width: i32, height: i32, color: Color);
        pub fn DrawRectangleLinesEx(rec: Rectangle, thick: f32, color: Color);
        pub fn DrawRectangleRounded(rec: Rectangle, roundness: f32, segments: i32, color: Color);
        pub fn DrawRectangleRoundedLines(rec: Rectangle, roundness: f32, segments: i32, color: Color);
        pub fn DrawRectangleRoundedLinesEx(rec: Rectangle, roundness: f32, segments: i32, thick: f32, color: Color);
        pub fn DrawPoly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color);
        pub fn DrawPolyLines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color);
        pub fn DrawPolyLinesEx(center: Vector2, sides: i32, radius: f32, rotation: f32, thick: f32, color: Color);
        pub fn DrawCircle(centerX: i32, centerY: i32, radius: f32, color: Color);
        pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
        pub fn DrawCircleGradient(center: Vector2, radius: f32, inner: Color, outer: Color);
        pub fn DrawCircleSector(center: Vector2, radius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color);
        pub fn DrawCircleSectorLines(center: Vector2, radius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color);
        pub fn DrawCircleLines(centerX: i32, centerY: i32, radius: f32, color: Color);
        pub fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color);
        pub fn DrawCircleLinesEx(center: Vector2, radius: f32, thick: f32, color: Color);
        pub fn DrawEllipse(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color);
        pub fn DrawEllipseV(center: Vector2, radiusH: f32, radiusV: f32, color: Color);
        pub fn DrawEllipseLines(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color);
        pub fn DrawEllipseLinesV(center: Vector2, radiusH: f32, radiusV: f32, color: Color);
        pub fn DrawRing(center: Vector2, innerRadius: f32, outerRadius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color);
        pub fn DrawRingLines(center: Vector2, innerRadius: f32, outerRadius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color);
        pub fn DrawSplineLinear(points: *const Vector2, pointCount: i32, thick: f32, color: Color);
        pub fn DrawSplineBasis(points: *const Vector2, pointCount: i32, thick: f32, color: Color);
        pub fn DrawSplineCatmullRom(points: *const Vector2, pointCount: i32, thick: f32, color: Color);
        pub fn DrawSplineBezierQuadratic(points: *const Vector2, pointCount: i32, thick: f32, color: Color);
        pub fn DrawSplineBezierCubic(points: *const Vector2, pointCount: i32, thick: f32, color: Color);
        pub fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color);
        pub fn DrawSplineSegmentBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color);
        pub fn DrawSplineSegmentCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color);
        pub fn DrawSplineSegmentBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, thick: f32, color: Color);
        pub fn DrawSplineSegmentBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, thick: f32, color: Color);
        pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32)->Vector2;
        pub fn GetSplinePointBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32)->Vector2;
        pub fn GetSplinePointCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32)->Vector2;
        pub fn GetSplinePointBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, t: f32)->Vector2;
        pub fn GetSplinePointBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, t: f32)->Vector2;
        pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle)->bool;
        pub fn CheckCollisionCircles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32)->bool;
        pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle)->bool;
        pub fn CheckCollisionCircleLine(center: Vector2, radius: f32, p1: Vector2, p2: Vector2)->bool;
        pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle)->bool;
        pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32)->bool;
        pub fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2)->bool;
        pub fn CheckCollisionPointLine(point: Vector2, p1: Vector2, p2: Vector2, threshold: i32)->bool;
        pub fn CheckCollisionPointPoly(point: Vector2, points: *const Vector2, pointCount: i32)->bool;
        pub fn CheckCollisionLines(startPos1: Vector2, endPos1: Vector2, startPos2: Vector2, endPos2: Vector2, collisionPoint: *mut Vector2)->bool;
        pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle)->Rectangle;
        pub fn LoadImage(fileName: *const c_char)->Image;
        pub fn LoadImageRaw(fileName: *const c_char, width: i32, height: i32, format: i32, headerSize: i32)->Image;
        pub fn LoadImageAnim(fileName: *const c_char, frames: *mut i32)->Image;
        pub fn LoadImageAnimFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: i32, frames: *mut i32)->Image;
        pub fn LoadImageFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: i32)->Image;
        pub fn LoadImageFromTexture(texture: Texture2D)->Image;
        pub fn LoadImageFromScreen()->Image;
        pub fn IsImageValid(image: Image)->bool;
        pub fn UnloadImage(image: Image);
        pub fn ExportImage(image: Image, fileName: *const c_char)->bool;
        pub fn ExportImageToMemory(image: Image, fileType: *const c_char, fileSize: *mut i32)->*mut c_uchar;
        pub fn ExportImageAsCode(image: Image, fileName: *const c_char)->bool;
        pub fn GenImageColor(width: i32, height: i32, color: Color)->Image;
        pub fn GenImageGradientLinear(width: i32, height: i32, direction: i32, start: Color, end: Color)->Image;
        pub fn GenImageGradientRadial(width: i32, height: i32, density: f32, inner: Color, outer: Color)->Image;
        pub fn GenImageGradientSquare(width: i32, height: i32, density: f32, inner: Color, outer: Color)->Image;
        pub fn GenImageChecked(width: i32, height: i32, checksX: i32, checksY: i32, col1: Color, col2: Color)->Image;
        pub fn GenImageWhiteNoise(width: i32, height: i32, factor: f32)->Image;
        pub fn GenImagePerlinNoise(width: i32, height: i32, offsetX: i32, offsetY: i32, scale: f32)->Image;
        pub fn GenImageCellular(width: i32, height: i32, tileSize: i32)->Image;
        pub fn GenImageText(width: i32, height: i32, text: *const c_char)->Image;
        pub fn ImageCopy(image: Image)->Image;
        pub fn ImageFromImage(image: Image, rec: Rectangle)->Image;
        pub fn ImageFromChannel(image: Image, selectedChannel: i32)->Image;
        pub fn ImageText(text: *const c_char, fontSize: i32, color: Color)->Image;
        pub fn ImageTextEx(font: Font, text: *const c_char, fontSize: f32, spacing: f32, tint: Color)->Image;
        pub fn ImageFormat(image: *mut Image, newFormat: i32);
        pub fn ImageToPOT(image: *mut Image, fill: Color);
        pub fn ImageCrop(image: *mut Image, crop: Rectangle);
        pub fn ImageAlphaCrop(image: *mut Image, threshold: f32);
        pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32);
        pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
        pub fn ImageAlphaPremultiply(image: *mut Image);
        pub fn ImageBlurGaussian(image: *mut Image, blurSize: i32);
        pub fn ImageKernelConvolution(image: *mut Image, kernel: *const f32, kernelSize: i32);
        pub fn ImageResize(image: *mut Image, newWidth: i32, newHeight: i32);
        pub fn ImageResizeNN(image: *mut Image, newWidth: i32, newHeight: i32);
        pub fn ImageResizeCanvas(image: *mut Image, newWidth: i32, newHeight: i32, offsetX: i32, offsetY: i32, fill: Color);
        pub fn ImageMipmaps(image: *mut Image);
        pub fn ImageDither(image: *mut Image, rBpp: i32, gBpp: i32, bBpp: i32, aBpp: i32);
        pub fn ImageFlipVertical(image: *mut Image);
        pub fn ImageFlipHorizontal(image: *mut Image);
        pub fn ImageRotate(image: *mut Image, degrees: i32);
        pub fn ImageRotateCW(image: *mut Image);
        pub fn ImageRotateCCW(image: *mut Image);
        pub fn ImageColorTint(image: *mut Image, color: Color);
        pub fn ImageColorInvert(image: *mut Image);
        pub fn ImageColorGrayscale(image: *mut Image);
        pub fn ImageColorContrast(image: *mut Image, contrast: i32);
        pub fn ImageColorBrightness(image: *mut Image, brightness: i32);
        pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
        pub fn LoadImageColors(image: Image)->*mut Color;
        pub fn LoadImagePalette(image: Image, maxPaletteSize: i32, colorCount: *mut i32)->*mut Color;
        pub fn UnloadImageColors(colors: *mut Color);
        pub fn UnloadImagePalette(colors: *mut Color);
        pub fn GetImageAlphaBorder(image: Image, threshold: f32)->Rectangle;
        pub fn GetImageColor(image: Image, x: i32, y: i32)->Color;
        pub fn ImageClearBackground(dst: *mut Image, color: Color);
        pub fn ImageDrawPixel(dst: *mut Image, posX: i32, posY: i32, color: Color);
        pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
        pub fn ImageDrawLine(dst: *mut Image, startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color);
        pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
        pub fn ImageDrawLineEx(dst: *mut Image, start: Vector2, end: Vector2, thick: i32, color: Color);
        pub fn ImageDrawLineStrip(dst: *mut Image, points: *const Vector2, pointCount: i32, color: Color);
        pub fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
        pub fn ImageDrawTriangleGradient(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color);
        pub fn ImageDrawTriangleLines(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
        pub fn ImageDrawTriangleFan(dst: *mut Image, points: *const Vector2, pointCount: i32, color: Color);
        pub fn ImageDrawTriangleStrip(dst: *mut Image, points: *const Vector2, pointCount: i32, color: Color);
        pub fn ImageDrawRectangle(dst: *mut Image, posX: i32, posY: i32, width: i32, height: i32, color: Color);
        pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
        pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
        pub fn ImageDrawRectanglePro(dst: *mut Image, rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
        pub fn ImageDrawRectangleLines(dst: *mut Image, posX: i32, posY: i32, width: i32, height: i32, color: Color);
        pub fn ImageDrawRectangleLinesEx(dst: *mut Image, rec: Rectangle, thick: i32, color: Color);
        pub fn ImageDrawRectangleGradientEx(dst: *mut Image, rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color);
        pub fn ImageDrawCircle(dst: *mut Image, centerX: i32, centerY: i32, radius: i32, color: Color);
        pub fn ImageDrawCircleV(dst: *mut Image, center: Vector2, radius: i32, color: Color);
        pub fn ImageDrawCircleLines(dst: *mut Image, centerX: i32, centerY: i32, radius: i32, color: Color);
        pub fn ImageDrawCircleLinesV(dst: *mut Image, center: Vector2, radius: i32, color: Color);
        pub fn ImageDrawCircleGradient(dst: *mut Image, center: Vector2, radius: f32, inner: Color, outer: Color);
        pub fn ImageDrawImage(dst: *mut Image, src: Image, posX: i32, posY: i32, tint: Color);
        pub fn ImageDrawImageEx(dst: *mut Image, src: Image, position: Vector2, rotation: f32, scale: f32, tint: Color);
        pub fn ImageDrawImageRec(dst: *mut Image, src: Image, srcRec: Rectangle, position: Vector2, tint: Color);
        pub fn ImageDrawImagePro(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle, origin: Vector2, rotation: f32, tint: Color);
        pub fn ImageDrawText(dst: *mut Image, text: *const c_char, posX: i32, posY: i32, fontSize: i32, color: Color);
        pub fn ImageDrawTextEx(dst: *mut Image, font: Font, text: *const c_char, position: Vector2, fontSize: f32, spacing: f32, tint: Color);
        pub fn ImageDrawTextPro(dst: *mut Image, font: Font, text: *const c_char, position: Vector2, origin: Vector2, rotation: f32, fontSize: f32, spacing: f32, tint: Color);
        pub fn LoadTexture(fileName: *const c_char)->Texture2D;
        pub fn LoadTextureFromImage(image: Image)->Texture2D;
        pub fn LoadTextureCubemap(image: Image, layout: i32)->TextureCubemap;
        pub fn LoadRenderTexture(width: i32, height: i32)->RenderTexture2D;
        pub fn IsTextureValid(texture: Texture2D)->bool;
        pub fn UnloadTexture(texture: Texture2D);
        pub fn IsRenderTextureValid(target: RenderTexture2D)->bool;
        pub fn UnloadRenderTexture(target: RenderTexture2D);
        pub fn UpdateTexture(texture: Texture2D, pixels: *const c_void);
        pub fn UpdateTextureRec(texture: Texture2D, rec: Rectangle, pixels: *const c_void);
        pub fn GenTextureMipmaps(texture: *mut Texture2D);
        pub fn SetTextureFilter(texture: Texture2D, filter: i32);
        pub fn SetTextureWrap(texture: Texture2D, wrap: i32);
        pub fn DrawTexture(texture: Texture2D, posX: i32, posY: i32, tint: Color);
        pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
        pub fn DrawTextureEx(texture: Texture2D, position: Vector2, rotation: f32, scale: f32, tint: Color);
        pub fn DrawTextureRec(texture: Texture2D, rec: Rectangle, position: Vector2, tint: Color);
        pub fn DrawTexturePro(texture: Texture2D, srcrec: Rectangle, dstrec: Rectangle, origin: Vector2, rotation: f32, tint: Color);
        pub fn DrawTextureNPatch(texture: Texture2D, nPatchInfo: NPatchInfo, dstrec: Rectangle, origin: Vector2, rotation: f32, tint: Color);
        pub fn ColorIsEqual(col1: Color, col2: Color)->bool;
        pub fn Fade(color: Color, alpha: f32)->Color;
        pub fn ColorToInt(color: Color)->i32;
        pub fn ColorNormalize(color: Color)->Vector4;
        pub fn ColorFromNormalized(normalized: Vector4)->Color;
        pub fn ColorToHSV(color: Color)->Vector3;
        pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32)->Color;
        pub fn ColorTint(color: Color, tint: Color)->Color;
        pub fn ColorBrightness(color: Color, factor: f32)->Color;
        pub fn ColorContrast(color: Color, contrast: f32)->Color;
        pub fn ColorAlpha(color: Color, alpha: f32)->Color;
        pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color)->Color;
        pub fn ColorLerp(color1: Color, color2: Color, factor: f32)->Color;
        pub fn GetColor(hexValue: u32)->Color;
        pub fn GetPixelColor(srcPtr: *const c_void, format: i32)->Color;
        pub fn SetPixelColor(dstPtr: *mut c_void, color: Color, format: i32);
        pub fn GetPixelDataSize(width: i32, height: i32, format: i32)->i32;
        pub fn GetFontDefault()->Font;
        pub fn LoadFont(fileName: *const c_char)->Font;
        pub fn LoadFontEx(fileName: *const c_char, fontSize: i32, codepoints: *const i32, codepointCount: i32)->Font;
        pub fn LoadFontFromImage(image: Image, key: Color, firstChar: i32)->Font;
        pub fn LoadFontFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: i32, fontSize: i32, codepoints: *const i32, codepointCount: i32)->Font;
        pub fn IsFontValid(font: Font)->bool;
        pub fn LoadFontData(fileData: *const c_uchar, dataSize: i32, fontSize: i32, codepoints: *const i32, codepointCount: i32, r#type: i32, glyphCount: *mut i32)->*mut GlyphInfo;
        pub fn GenImageFontAtlas(glyphs: *const GlyphInfo, glyphRecs: *mut Rectangle, glyphCount: i32, fontSize: i32, padding: i32, packMethod: i32)->Image;
        pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: i32);
        pub fn UnloadFont(font: Font);
        pub fn ExportFontAsCode(font: Font, fileName: *const c_char)->bool;
        pub fn DrawFPS(posX: i32, posY: i32);
        pub fn DrawText(text: *const c_char, posX: i32, posY: i32, fontSize: i32, color: Color);
        pub fn DrawTextEx(font: Font, text: *const c_char, position: Vector2, fontSize: f32, spacing: f32, tint: Color);
        pub fn DrawTextPro(font: Font, text: *const c_char, position: Vector2, origin: Vector2, rotation: f32, fontSize: f32, spacing: f32, tint: Color);
        pub fn DrawTextCodepoint(font: Font, codepoint: i32, position: Vector2, fontSize: f32, tint: Color);
        pub fn DrawTextCodepoints(font: Font, codepoints: *const i32, codepointCount: i32, position: Vector2, fontSize: f32, spacing: f32, tint: Color);
        pub fn SetTextLineSpacing(spacing: i32);
        pub fn MeasureText(text: *const c_char, fontSize: i32)->i32;
        pub fn MeasureTextEx(font: Font, text: *const c_char, fontSize: f32, spacing: f32)->Vector2;
        pub fn MeasureTextCodepoints(font: Font, codepoints: *const i32, length: i32, fontSize: f32, spacing: f32)->Vector2;
        pub fn GetGlyphIndex(font: Font, codepoint: i32)->i32;
        pub fn GetGlyphInfo(font: Font, codepoint: i32)->GlyphInfo;
        pub fn GetGlyphAtlasRec(font: Font, codepoint: i32)->Rectangle;
        pub fn LoadUTF8(codepoints: *const i32, length: i32)->*mut c_char;
        pub fn UnloadUTF8(text: *mut c_char);
        pub fn LoadCodepoints(text: *const c_char, count: *mut i32)->*mut i32;
        pub fn UnloadCodepoints(codepoints: *mut i32);
        pub fn GetCodepointCount(text: *const c_char)->i32;
        pub fn GetCodepoint(text: *const c_char, codepointSize: *mut i32)->i32;
        pub fn GetCodepointNext(text: *const c_char, codepointSize: *mut i32)->i32;
        pub fn GetCodepointPrevious(text: *const c_char, codepointSize: *mut i32)->i32;
        pub fn CodepointToUTF8(codepoint: i32, utf8Size: *mut i32)->*const c_char;
        pub fn LoadTextLines(text: *const c_char, count: *mut i32)->*mut c_char;
        pub fn UnloadTextLines(text: *mut c_char, lineCount: i32);
        pub fn TextCopy(dst: *mut c_char, src: *const c_char)->i32;
        pub fn TextIsEqual(text1: *const c_char, text2: *const c_char)->bool;
        pub fn TextLength(text: *const c_char)->u32;
        pub fn TextFormat(text: *const c_char, ...)->*const c_char;
        pub fn TextSubtext(text: *const c_char, position: i32, length: i32)->*const c_char;
        pub fn TextRemoveSpaces(text: *const c_char)->*const c_char;
        pub fn GetTextBetween(text: *const c_char, begin: *const c_char, end: *const c_char)->*mut c_char;
        pub fn TextReplace(text: *const c_char, search: *const c_char, replacement: *const c_char)->*mut c_char;
        pub fn TextReplaceAlloc(text: *const c_char, search: *const c_char, replacement: *const c_char)->*mut c_char;
        pub fn TextReplaceBetween(text: *const c_char, begin: *const c_char, end: *const c_char, replacement: *const c_char)->*mut c_char;
        pub fn TextReplaceBetweenAlloc(text: *const c_char, begin: *const c_char, end: *const c_char, replacement: *const c_char)->*mut c_char;
        pub fn TextInsert(text: *const c_char, insert: *const c_char, position: i32)->*mut c_char;
        pub fn TextInsertAlloc(text: *const c_char, insert: *const c_char, position: i32)->*mut c_char;
        pub fn TextJoin(textList: *mut c_char, count: i32, delimiter: *const c_char)->*mut c_char;
        pub fn TextSplit(text: *const c_char, delimiter: i8, count: *mut i32)->*mut c_char;
        pub fn TextAppend(text: *mut c_char, append: *const c_char, position: *mut i32);
        pub fn TextFindIndex(text: *const c_char, search: *const c_char)->i32;
        pub fn TextToUpper(text: *const c_char)->*mut c_char;
        pub fn TextToLower(text: *const c_char)->*mut c_char;
        pub fn TextToPascal(text: *const c_char)->*mut c_char;
        pub fn TextToSnake(text: *const c_char)->*mut c_char;
        pub fn TextToCamel(text: *const c_char)->*mut c_char;
        pub fn TextToInteger(text: *const c_char)->i32;
        pub fn TextToFloat(text: *const c_char)->f32;
        pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
        pub fn DrawPoint3D(position: Vector3, color: Color);
        pub fn DrawCircle3D(center: Vector3, radius: f32, rotationAxis: Vector3, rotationAngle: f32, color: Color);
        pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
        pub fn DrawTriangleStrip3D(points: *const Vector3, pointCount: i32, color: Color);
        pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
        pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
        pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);
        pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
        pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color);
        pub fn DrawSphereEx(centerPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color);
        pub fn DrawSphereWires(centerPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color);
        pub fn DrawCylinder(position: Vector3, radiusTop: f32, radiusBottom: f32, height: f32, sides: i32, color: Color);
        pub fn DrawCylinderEx(startPos: Vector3, endPos: Vector3, startRadius: f32, endRadius: f32, sides: i32, color: Color);
        pub fn DrawCylinderWires(position: Vector3, radiusTop: f32, radiusBottom: f32, height: f32, sides: i32, color: Color);
        pub fn DrawCylinderWiresEx(startPos: Vector3, endPos: Vector3, startRadius: f32, endRadius: f32, sides: i32, color: Color);
        pub fn DrawCapsule(startPos: Vector3, endPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color);
        pub fn DrawCapsuleWires(startPos: Vector3, endPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color);
        pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
        pub fn DrawRay(ray: Ray, color: Color);
        pub fn DrawGrid(slices: i32, spacing: f32);
        pub fn LoadModel(fileName: *const c_char)->Model;
        pub fn LoadModelFromMesh(mesh: Mesh)->Model;
        pub fn IsModelValid(model: Model)->bool;
        pub fn UnloadModel(model: Model);
        pub fn GetModelBoundingBox(model: Model)->BoundingBox;
        pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color);
        pub fn DrawModelEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: f32, scale: Vector3, tint: Color);
        pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color);
        pub fn DrawModelWiresEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: f32, scale: Vector3, tint: Color);
        pub fn DrawBoundingBox(r#box: BoundingBox, color: Color);
        pub fn DrawBillboard(camera: Camera, texture: Texture2D, position: Vector3, scale: f32, tint: Color);
        pub fn DrawBillboardRec(camera: Camera, texture: Texture2D, rec: Rectangle, position: Vector3, size: Vector2, tint: Color);
        pub fn DrawBillboardPro(camera: Camera, texture: Texture2D, rec: Rectangle, position: Vector3, up: Vector3, size: Vector2, origin: Vector2, rotation: f32, tint: Color);
        pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
        pub fn UpdateMeshBuffer(mesh: Mesh, index: i32, data: *const c_void, dataSize: i32, offset: i32);
        pub fn UnloadMesh(mesh: Mesh);
        pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
        pub fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: *const Matrix, instances: i32);
        pub fn GetMeshBoundingBox(mesh: Mesh)->BoundingBox;
        pub fn GenMeshTangents(mesh: *mut Mesh);
        pub fn ExportMesh(mesh: Mesh, fileName: *const c_char)->bool;
        pub fn ExportMeshAsCode(mesh: Mesh, fileName: *const c_char)->bool;
        pub fn GenMeshPoly(sides: i32, radius: f32)->Mesh;
        pub fn GenMeshPlane(width: f32, length: f32, resX: i32, resZ: i32)->Mesh;
        pub fn GenMeshCube(width: f32, height: f32, length: f32)->Mesh;
        pub fn GenMeshSphere(radius: f32, rings: i32, slices: i32)->Mesh;
        pub fn GenMeshHemiSphere(radius: f32, rings: i32, slices: i32)->Mesh;
        pub fn GenMeshCylinder(radius: f32, height: f32, slices: i32)->Mesh;
        pub fn GenMeshCone(radius: f32, height: f32, slices: i32)->Mesh;
        pub fn GenMeshTorus(radius: f32, size: f32, radSeg: i32, sides: i32)->Mesh;
        pub fn GenMeshKnot(radius: f32, size: f32, radSeg: i32, sides: i32)->Mesh;
        pub fn GenMeshHeightmap(heightmap: Image, size: Vector3)->Mesh;
        pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3)->Mesh;
        pub fn LoadMaterials(fileName: *const c_char, materialCount: *mut i32)->*mut Material;
        pub fn LoadMaterialDefault()->Material;
        pub fn IsMaterialValid(material: Material)->bool;
        pub fn UnloadMaterial(material: Material);
        pub fn SetMaterialTexture(material: *mut Material, mapType: i32, texture: Texture2D);
        pub fn SetModelMeshMaterial(model: *mut Model, meshId: i32, materialId: i32);
        pub fn LoadModelAnimations(fileName: *const c_char, animCount: *mut i32)->*mut ModelAnimation;
        pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: f32);
        pub fn UpdateModelAnimationEx(model: Model, animA: ModelAnimation, frameA: f32, animB: ModelAnimation, frameB: f32, blend: f32);
        pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: i32);
        pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation)->bool;
        pub fn CheckCollisionSpheres(center1: Vector3, radius1: f32, center2: Vector3, radius2: f32)->bool;
        pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox)->bool;
        pub fn CheckCollisionBoxSphere(r#box: BoundingBox, center: Vector3, radius: f32)->bool;
        pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: f32)->RayCollision;
        pub fn GetRayCollisionBox(ray: Ray, r#box: BoundingBox)->RayCollision;
        pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix)->RayCollision;
        pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)->RayCollision;
        pub fn GetRayCollisionQuad(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3)->RayCollision;
        pub fn InitAudioDevice();
        pub fn CloseAudioDevice();
        pub fn IsAudioDeviceReady()->bool;
        pub fn SetMasterVolume(volume: f32);
        pub fn GetMasterVolume()->f32;
        pub fn LoadWave(fileName: *const c_char)->Wave;
        pub fn LoadWaveFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: i32)->Wave;
        pub fn IsWaveValid(wave: Wave)->bool;
        pub fn LoadSound(fileName: *const c_char)->Sound;
        pub fn LoadSoundFromWave(wave: Wave)->Sound;
        pub fn LoadSoundAlias(source: Sound)->Sound;
        pub fn IsSoundValid(sound: Sound)->bool;
        pub fn UpdateSound(sound: Sound, data: *const c_void, frameCount: i32);
        pub fn UnloadWave(wave: Wave);
        pub fn UnloadSound(sound: Sound);
        pub fn UnloadSoundAlias(alias: Sound);
        pub fn ExportWave(wave: Wave, fileName: *const c_char)->bool;
        pub fn ExportWaveAsCode(wave: Wave, fileName: *const c_char)->bool;
        pub fn PlaySound(sound: Sound);
        pub fn StopSound(sound: Sound);
        pub fn PauseSound(sound: Sound);
        pub fn ResumeSound(sound: Sound);
        pub fn IsSoundPlaying(sound: Sound)->bool;
        pub fn SetSoundVolume(sound: Sound, volume: f32);
        pub fn SetSoundPitch(sound: Sound, pitch: f32);
        pub fn SetSoundPan(sound: Sound, pan: f32);
        pub fn WaveCopy(wave: Wave)->Wave;
        pub fn WaveCrop(wave: *mut Wave, initFrame: i32, finalFrame: i32);
        pub fn WaveFormat(wave: *mut Wave, sampleRate: i32, sampleSize: i32, channels: i32);
        pub fn LoadWaveSamples(wave: Wave)->*mut f32;
        pub fn UnloadWaveSamples(samples: *mut f32);
        pub fn LoadMusicStream(fileName: *const c_char)->Music;
        pub fn LoadMusicStreamFromMemory(fileType: *const c_char, data: *const c_uchar, dataSize: i32)->Music;
        pub fn IsMusicValid(music: Music)->bool;
        pub fn UnloadMusicStream(music: Music);
        pub fn PlayMusicStream(music: Music);
        pub fn IsMusicStreamPlaying(music: Music)->bool;
        pub fn UpdateMusicStream(music: Music);
        pub fn StopMusicStream(music: Music);
        pub fn PauseMusicStream(music: Music);
        pub fn ResumeMusicStream(music: Music);
        pub fn SeekMusicStream(music: Music, position: f32);
        pub fn SetMusicVolume(music: Music, volume: f32);
        pub fn SetMusicPitch(music: Music, pitch: f32);
        pub fn SetMusicPan(music: Music, pan: f32);
        pub fn GetMusicTimeLength(music: Music)->f32;
        pub fn GetMusicTimePlayed(music: Music)->f32;
        pub fn LoadAudioStream(sampleRate: u32, sampleSize: u32, channels: u32)->AudioStream;
        pub fn IsAudioStreamValid(stream: AudioStream)->bool;
        pub fn UnloadAudioStream(stream: AudioStream);
        pub fn UpdateAudioStream(stream: AudioStream, data: *const c_void, frameCount: i32);
        pub fn IsAudioStreamProcessed(stream: AudioStream)->bool;
        pub fn PlayAudioStream(stream: AudioStream);
        pub fn PauseAudioStream(stream: AudioStream);
        pub fn ResumeAudioStream(stream: AudioStream);
        pub fn IsAudioStreamPlaying(stream: AudioStream)->bool;
        pub fn StopAudioStream(stream: AudioStream);
        pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32);
        pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32);
        pub fn SetAudioStreamPan(stream: AudioStream, pan: f32);
        pub fn SetAudioStreamBufferSizeDefault(size: i32);
    }}
pub fn InitWindow(width: i32, height: i32, title: impl AsRef<str>){unsafe{raw::InitWindow(width, height, CString::new(title.as_ref()).unwrap().as_ptr())}}
pub fn CloseWindow(){unsafe{raw::CloseWindow()}}
pub fn WindowShouldClose()->bool{unsafe{raw::WindowShouldClose()}}
pub fn IsWindowReady()->bool{unsafe{raw::IsWindowReady()}}
pub fn IsWindowFullscreen()->bool{unsafe{raw::IsWindowFullscreen()}}
pub fn IsWindowHidden()->bool{unsafe{raw::IsWindowHidden()}}
pub fn IsWindowMinimized()->bool{unsafe{raw::IsWindowMinimized()}}
pub fn IsWindowMaximized()->bool{unsafe{raw::IsWindowMaximized()}}
pub fn IsWindowFocused()->bool{unsafe{raw::IsWindowFocused()}}
pub fn IsWindowResized()->bool{unsafe{raw::IsWindowResized()}}
pub fn IsWindowState(flag: u32)->bool{unsafe{raw::IsWindowState(flag)}}
pub fn SetWindowState(flags: u32){unsafe{raw::SetWindowState(flags)}}
pub fn ClearWindowState(flags: u32){unsafe{raw::ClearWindowState(flags)}}
pub fn ToggleFullscreen(){unsafe{raw::ToggleFullscreen()}}
pub fn ToggleBorderlessWindowed(){unsafe{raw::ToggleBorderlessWindowed()}}
pub fn MaximizeWindow(){unsafe{raw::MaximizeWindow()}}
pub fn MinimizeWindow(){unsafe{raw::MinimizeWindow()}}
pub fn RestoreWindow(){unsafe{raw::RestoreWindow()}}
pub fn SetWindowIcon(image: Image){unsafe{raw::SetWindowIcon(image)}}
pub fn SetWindowIcons(images: *mut Image, count: i32){unsafe{raw::SetWindowIcons(images, count)}}
pub fn SetWindowTitle(title: impl AsRef<str>){unsafe{raw::SetWindowTitle(CString::new(title.as_ref()).unwrap().as_ptr())}}
pub fn SetWindowPosition(x: i32, y: i32){unsafe{raw::SetWindowPosition(x, y)}}
pub fn SetWindowMonitor(monitor: i32){unsafe{raw::SetWindowMonitor(monitor)}}
pub fn SetWindowMinSize(width: i32, height: i32){unsafe{raw::SetWindowMinSize(width, height)}}
pub fn SetWindowMaxSize(width: i32, height: i32){unsafe{raw::SetWindowMaxSize(width, height)}}
pub fn SetWindowSize(width: i32, height: i32){unsafe{raw::SetWindowSize(width, height)}}
pub fn SetWindowOpacity(opacity: f32){unsafe{raw::SetWindowOpacity(opacity)}}
pub fn SetWindowFocused(){unsafe{raw::SetWindowFocused()}}
pub fn GetWindowHandle()->*mut c_void{unsafe{raw::GetWindowHandle()}}
pub fn GetScreenWidth()->i32{unsafe{raw::GetScreenWidth()}}
pub fn GetScreenHeight()->i32{unsafe{raw::GetScreenHeight()}}
pub fn GetRenderWidth()->i32{unsafe{raw::GetRenderWidth()}}
pub fn GetRenderHeight()->i32{unsafe{raw::GetRenderHeight()}}
pub fn GetMonitorCount()->i32{unsafe{raw::GetMonitorCount()}}
pub fn GetCurrentMonitor()->i32{unsafe{raw::GetCurrentMonitor()}}
pub fn GetMonitorPosition(monitor: i32)->Vector2{unsafe{raw::GetMonitorPosition(monitor)}}
pub fn GetMonitorWidth(monitor: i32)->i32{unsafe{raw::GetMonitorWidth(monitor)}}
pub fn GetMonitorHeight(monitor: i32)->i32{unsafe{raw::GetMonitorHeight(monitor)}}
pub fn GetMonitorPhysicalWidth(monitor: i32)->i32{unsafe{raw::GetMonitorPhysicalWidth(monitor)}}
pub fn GetMonitorPhysicalHeight(monitor: i32)->i32{unsafe{raw::GetMonitorPhysicalHeight(monitor)}}
pub fn GetMonitorRefreshRate(monitor: i32)->i32{unsafe{raw::GetMonitorRefreshRate(monitor)}}
pub fn GetWindowPosition()->Vector2{unsafe{raw::GetWindowPosition()}}
pub fn GetWindowScaleDPI()->Vector2{unsafe{raw::GetWindowScaleDPI()}}
pub fn GetMonitorName(monitor: i32)->*const c_char{unsafe{raw::GetMonitorName(monitor)}}
pub fn SetClipboardText(text: impl AsRef<str>){unsafe{raw::SetClipboardText(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn GetClipboardText()->*const c_char{unsafe{raw::GetClipboardText()}}
pub fn GetClipboardImage()->Image{unsafe{raw::GetClipboardImage()}}
pub fn EnableEventWaiting(){unsafe{raw::EnableEventWaiting()}}
pub fn DisableEventWaiting(){unsafe{raw::DisableEventWaiting()}}
pub fn ShowCursor(){unsafe{raw::ShowCursor()}}
pub fn HideCursor(){unsafe{raw::HideCursor()}}
pub fn IsCursorHidden()->bool{unsafe{raw::IsCursorHidden()}}
pub fn EnableCursor(){unsafe{raw::EnableCursor()}}
pub fn DisableCursor(){unsafe{raw::DisableCursor()}}
pub fn IsCursorOnScreen()->bool{unsafe{raw::IsCursorOnScreen()}}
pub fn ClearBackground(color: Color){unsafe{raw::ClearBackground(color)}}
pub fn BeginDrawing(){unsafe{raw::BeginDrawing()}}
pub fn EndDrawing(){unsafe{raw::EndDrawing()}}
pub fn BeginMode2D(camera: Camera2D){unsafe{raw::BeginMode2D(camera)}}
pub fn EndMode2D(){unsafe{raw::EndMode2D()}}
pub fn BeginMode3D(camera: Camera3D){unsafe{raw::BeginMode3D(camera)}}
pub fn EndMode3D(){unsafe{raw::EndMode3D()}}
pub fn BeginTextureMode(target: RenderTexture2D){unsafe{raw::BeginTextureMode(target)}}
pub fn EndTextureMode(){unsafe{raw::EndTextureMode()}}
pub fn BeginShaderMode(shader: Shader){unsafe{raw::BeginShaderMode(shader)}}
pub fn EndShaderMode(){unsafe{raw::EndShaderMode()}}
pub fn BeginBlendMode(mode: i32){unsafe{raw::BeginBlendMode(mode)}}
pub fn EndBlendMode(){unsafe{raw::EndBlendMode()}}
pub fn BeginScissorMode(x: i32, y: i32, width: i32, height: i32){unsafe{raw::BeginScissorMode(x, y, width, height)}}
pub fn EndScissorMode(){unsafe{raw::EndScissorMode()}}
pub fn BeginVrStereoMode(config: VrStereoConfig){unsafe{raw::BeginVrStereoMode(config)}}
pub fn EndVrStereoMode(){unsafe{raw::EndVrStereoMode()}}
pub fn LoadVrStereoConfig(device: VrDeviceInfo)->VrStereoConfig{unsafe{raw::LoadVrStereoConfig(device)}}
pub fn UnloadVrStereoConfig(config: VrStereoConfig){unsafe{raw::UnloadVrStereoConfig(config)}}
pub fn LoadShader(vsFileName: impl AsRef<str>, fsFileName: impl AsRef<str>)->Shader{unsafe{raw::LoadShader(CString::new(vsFileName.as_ref()).unwrap().as_ptr(), CString::new(fsFileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadShaderFromMemory(vsCode: impl AsRef<str>, fsCode: impl AsRef<str>)->Shader{unsafe{raw::LoadShaderFromMemory(CString::new(vsCode.as_ref()).unwrap().as_ptr(), CString::new(fsCode.as_ref()).unwrap().as_ptr())}}
pub fn IsShaderValid(shader: Shader)->bool{unsafe{raw::IsShaderValid(shader)}}
pub fn GetShaderLocation(shader: Shader, uniformName: impl AsRef<str>)->i32{unsafe{raw::GetShaderLocation(shader, CString::new(uniformName.as_ref()).unwrap().as_ptr())}}
pub fn GetShaderLocationAttrib(shader: Shader, attribName: impl AsRef<str>)->i32{unsafe{raw::GetShaderLocationAttrib(shader, CString::new(attribName.as_ref()).unwrap().as_ptr())}}
pub fn SetShaderValue(shader: Shader, locIndex: i32, value: *const c_void, uniformType: i32){unsafe{raw::SetShaderValue(shader, locIndex, value, uniformType)}}
pub fn SetShaderValueV(shader: Shader, locIndex: i32, value: *const c_void, uniformType: i32, count: i32){unsafe{raw::SetShaderValueV(shader, locIndex, value, uniformType, count)}}
pub fn SetShaderValueMatrix(shader: Shader, locIndex: i32, mat: Matrix){unsafe{raw::SetShaderValueMatrix(shader, locIndex, mat)}}
pub fn SetShaderValueTexture(shader: Shader, locIndex: i32, texture: Texture2D){unsafe{raw::SetShaderValueTexture(shader, locIndex, texture)}}
pub fn UnloadShader(shader: Shader){unsafe{raw::UnloadShader(shader)}}
pub fn GetScreenToWorldRay(position: Vector2, camera: Camera)->Ray{unsafe{raw::GetScreenToWorldRay(position, camera)}}
pub fn GetScreenToWorldRayEx(position: Vector2, camera: Camera, width: i32, height: i32)->Ray{unsafe{raw::GetScreenToWorldRayEx(position, camera, width, height)}}
pub fn GetWorldToScreen(position: Vector3, camera: Camera)->Vector2{unsafe{raw::GetWorldToScreen(position, camera)}}
pub fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: i32, height: i32)->Vector2{unsafe{raw::GetWorldToScreenEx(position, camera, width, height)}}
pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D)->Vector2{unsafe{raw::GetWorldToScreen2D(position, camera)}}
pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D)->Vector2{unsafe{raw::GetScreenToWorld2D(position, camera)}}
pub fn GetCameraMatrix(camera: Camera)->Matrix{unsafe{raw::GetCameraMatrix(camera)}}
pub fn GetCameraMatrix2D(camera: Camera2D)->Matrix{unsafe{raw::GetCameraMatrix2D(camera)}}
pub fn SetTargetFPS(fps: i32){unsafe{raw::SetTargetFPS(fps)}}
pub fn GetFrameTime()->f32{unsafe{raw::GetFrameTime()}}
pub fn GetTime()->f64{unsafe{raw::GetTime()}}
pub fn GetFPS()->i32{unsafe{raw::GetFPS()}}
pub fn SwapScreenBuffer(){unsafe{raw::SwapScreenBuffer()}}
pub fn PollInputEvents(){unsafe{raw::PollInputEvents()}}
pub fn WaitTime(seconds: f64){unsafe{raw::WaitTime(seconds)}}
pub fn SetRandomSeed(seed: u32){unsafe{raw::SetRandomSeed(seed)}}
pub fn GetRandomValue(min: i32, max: i32)->i32{unsafe{raw::GetRandomValue(min, max)}}
pub fn LoadRandomSequence(count: u32, min: i32, max: i32)->*mut i32{unsafe{raw::LoadRandomSequence(count, min, max)}}
pub fn UnloadRandomSequence(sequence: *mut i32){unsafe{raw::UnloadRandomSequence(sequence)}}
pub fn TakeScreenshot(fileName: impl AsRef<str>){unsafe{raw::TakeScreenshot(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn SetConfigFlags(flags: u32){unsafe{raw::SetConfigFlags(flags)}}
pub fn OpenURL(url: impl AsRef<str>){unsafe{raw::OpenURL(CString::new(url.as_ref()).unwrap().as_ptr())}}
pub fn SetTraceLogLevel(logLevel: i32){unsafe{raw::SetTraceLogLevel(logLevel)}}
pub fn TraceLog(logLevel: i32, text: impl AsRef<str>){unsafe{raw::TraceLog(logLevel, CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn MemAlloc(size: u32)->*mut c_void{unsafe{raw::MemAlloc(size)}}
pub fn MemRealloc(ptr: *mut c_void, size: u32)->*mut c_void{unsafe{raw::MemRealloc(ptr, size)}}
pub fn MemFree(ptr: *mut c_void){unsafe{raw::MemFree(ptr)}}
pub fn LoadFileData(fileName: impl AsRef<str>, dataSize: *mut i32)->*mut c_uchar{unsafe{raw::LoadFileData(CString::new(fileName.as_ref()).unwrap().as_ptr(), dataSize)}}
pub fn UnloadFileData(data: *mut c_uchar){unsafe{raw::UnloadFileData(data)}}
pub fn SaveFileData(fileName: impl AsRef<str>, data: *const c_void, dataSize: i32)->bool{unsafe{raw::SaveFileData(CString::new(fileName.as_ref()).unwrap().as_ptr(), data, dataSize)}}
pub fn ExportDataAsCode(data: *const c_uchar, dataSize: i32, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportDataAsCode(data, dataSize, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadFileText(fileName: impl AsRef<str>)->*mut c_char{unsafe{raw::LoadFileText(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn UnloadFileText(text: *mut c_char){unsafe{raw::UnloadFileText(text)}}
pub fn SaveFileText(fileName: impl AsRef<str>, text: impl AsRef<str>)->bool{unsafe{raw::SaveFileText(CString::new(fileName.as_ref()).unwrap().as_ptr(), CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn FileRename(fileName: impl AsRef<str>, fileRename: impl AsRef<str>)->i32{unsafe{raw::FileRename(CString::new(fileName.as_ref()).unwrap().as_ptr(), CString::new(fileRename.as_ref()).unwrap().as_ptr())}}
pub fn FileRemove(fileName: impl AsRef<str>)->i32{unsafe{raw::FileRemove(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn FileCopy(srcPath: impl AsRef<str>, dstPath: impl AsRef<str>)->i32{unsafe{raw::FileCopy(CString::new(srcPath.as_ref()).unwrap().as_ptr(), CString::new(dstPath.as_ref()).unwrap().as_ptr())}}
pub fn FileMove(srcPath: impl AsRef<str>, dstPath: impl AsRef<str>)->i32{unsafe{raw::FileMove(CString::new(srcPath.as_ref()).unwrap().as_ptr(), CString::new(dstPath.as_ref()).unwrap().as_ptr())}}
pub fn FileTextReplace(fileName: impl AsRef<str>, search: impl AsRef<str>, replacement: impl AsRef<str>)->i32{unsafe{raw::FileTextReplace(CString::new(fileName.as_ref()).unwrap().as_ptr(), CString::new(search.as_ref()).unwrap().as_ptr(), CString::new(replacement.as_ref()).unwrap().as_ptr())}}
pub fn FileTextFindIndex(fileName: impl AsRef<str>, search: impl AsRef<str>)->i32{unsafe{raw::FileTextFindIndex(CString::new(fileName.as_ref()).unwrap().as_ptr(), CString::new(search.as_ref()).unwrap().as_ptr())}}
pub fn FileExists(fileName: impl AsRef<str>)->bool{unsafe{raw::FileExists(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn DirectoryExists(dirPath: impl AsRef<str>)->bool{unsafe{raw::DirectoryExists(CString::new(dirPath.as_ref()).unwrap().as_ptr())}}
pub fn IsFileExtension(fileName: impl AsRef<str>, ext: impl AsRef<str>)->bool{unsafe{raw::IsFileExtension(CString::new(fileName.as_ref()).unwrap().as_ptr(), CString::new(ext.as_ref()).unwrap().as_ptr())}}
pub fn GetFileLength(fileName: impl AsRef<str>)->i32{unsafe{raw::GetFileLength(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn GetFileModTime(fileName: impl AsRef<str>)->i64{unsafe{raw::GetFileModTime(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn GetFileExtension(fileName: impl AsRef<str>)->*const c_char{unsafe{raw::GetFileExtension(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn GetFileName(filePath: impl AsRef<str>)->*const c_char{unsafe{raw::GetFileName(CString::new(filePath.as_ref()).unwrap().as_ptr())}}
pub fn GetFileNameWithoutExt(filePath: impl AsRef<str>)->*const c_char{unsafe{raw::GetFileNameWithoutExt(CString::new(filePath.as_ref()).unwrap().as_ptr())}}
pub fn GetDirectoryPath(filePath: impl AsRef<str>)->*const c_char{unsafe{raw::GetDirectoryPath(CString::new(filePath.as_ref()).unwrap().as_ptr())}}
pub fn GetPrevDirectoryPath(dirPath: impl AsRef<str>)->*const c_char{unsafe{raw::GetPrevDirectoryPath(CString::new(dirPath.as_ref()).unwrap().as_ptr())}}
pub fn GetWorkingDirectory()->*const c_char{unsafe{raw::GetWorkingDirectory()}}
pub fn GetApplicationDirectory()->*const c_char{unsafe{raw::GetApplicationDirectory()}}
pub fn MakeDirectory(dirPath: impl AsRef<str>)->i32{unsafe{raw::MakeDirectory(CString::new(dirPath.as_ref()).unwrap().as_ptr())}}
pub fn ChangeDirectory(dirPath: impl AsRef<str>)->i32{unsafe{raw::ChangeDirectory(CString::new(dirPath.as_ref()).unwrap().as_ptr())}}
pub fn IsPathFile(path: impl AsRef<str>)->bool{unsafe{raw::IsPathFile(CString::new(path.as_ref()).unwrap().as_ptr())}}
pub fn IsPathDirectory(path: impl AsRef<str>)->bool{unsafe{raw::IsPathDirectory(CString::new(path.as_ref()).unwrap().as_ptr())}}
pub fn IsFileNameValid(fileName: impl AsRef<str>)->bool{unsafe{raw::IsFileNameValid(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadDirectoryFiles(dirPath: impl AsRef<str>)->FilePathList{unsafe{raw::LoadDirectoryFiles(CString::new(dirPath.as_ref()).unwrap().as_ptr())}}
pub fn LoadDirectoryFilesEx(basePath: impl AsRef<str>, filter: impl AsRef<str>, scanSubdirs: bool)->FilePathList{unsafe{raw::LoadDirectoryFilesEx(CString::new(basePath.as_ref()).unwrap().as_ptr(), CString::new(filter.as_ref()).unwrap().as_ptr(), scanSubdirs)}}
pub fn UnloadDirectoryFiles(files: FilePathList){unsafe{raw::UnloadDirectoryFiles(files)}}
pub fn IsFileDropped()->bool{unsafe{raw::IsFileDropped()}}
pub fn LoadDroppedFiles()->FilePathList{unsafe{raw::LoadDroppedFiles()}}
pub fn UnloadDroppedFiles(files: FilePathList){unsafe{raw::UnloadDroppedFiles(files)}}
pub fn GetDirectoryFileCount(dirPath: impl AsRef<str>)->u32{unsafe{raw::GetDirectoryFileCount(CString::new(dirPath.as_ref()).unwrap().as_ptr())}}
pub fn GetDirectoryFileCountEx(basePath: impl AsRef<str>, filter: impl AsRef<str>, scanSubdirs: bool)->u32{unsafe{raw::GetDirectoryFileCountEx(CString::new(basePath.as_ref()).unwrap().as_ptr(), CString::new(filter.as_ref()).unwrap().as_ptr(), scanSubdirs)}}
pub fn CompressData(data: *const c_uchar, dataSize: i32, compDataSize: *mut i32)->*mut c_uchar{unsafe{raw::CompressData(data, dataSize, compDataSize)}}
pub fn DecompressData(compData: *const c_uchar, compDataSize: i32, dataSize: *mut i32)->*mut c_uchar{unsafe{raw::DecompressData(compData, compDataSize, dataSize)}}
pub fn EncodeDataBase64(data: *const c_uchar, dataSize: i32, outputSize: *mut i32)->*mut c_char{unsafe{raw::EncodeDataBase64(data, dataSize, outputSize)}}
pub fn DecodeDataBase64(text: impl AsRef<str>, outputSize: *mut i32)->*mut c_uchar{unsafe{raw::DecodeDataBase64(CString::new(text.as_ref()).unwrap().as_ptr(), outputSize)}}
pub fn ComputeCRC32(data: *const c_uchar, dataSize: i32)->u32{unsafe{raw::ComputeCRC32(data, dataSize)}}
pub fn ComputeMD5(data: *const c_uchar, dataSize: i32)->*mut u32{unsafe{raw::ComputeMD5(data, dataSize)}}
pub fn ComputeSHA1(data: *const c_uchar, dataSize: i32)->*mut u32{unsafe{raw::ComputeSHA1(data, dataSize)}}
pub fn ComputeSHA256(data: *const c_uchar, dataSize: i32)->*mut u32{unsafe{raw::ComputeSHA256(data, dataSize)}}
pub fn LoadAutomationEventList(fileName: impl AsRef<str>)->AutomationEventList{unsafe{raw::LoadAutomationEventList(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn UnloadAutomationEventList(list: AutomationEventList){unsafe{raw::UnloadAutomationEventList(list)}}
pub fn ExportAutomationEventList(list: AutomationEventList, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportAutomationEventList(list, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn SetAutomationEventList(list: *mut AutomationEventList){unsafe{raw::SetAutomationEventList(list)}}
pub fn SetAutomationEventBaseFrame(frame: i32){unsafe{raw::SetAutomationEventBaseFrame(frame)}}
pub fn StartAutomationEventRecording(){unsafe{raw::StartAutomationEventRecording()}}
pub fn StopAutomationEventRecording(){unsafe{raw::StopAutomationEventRecording()}}
pub fn PlayAutomationEvent(event: AutomationEvent){unsafe{raw::PlayAutomationEvent(event)}}
pub fn IsKeyPressed(key: i32)->bool{unsafe{raw::IsKeyPressed(key)}}
pub fn IsKeyPressedRepeat(key: i32)->bool{unsafe{raw::IsKeyPressedRepeat(key)}}
pub fn IsKeyDown(key: i32)->bool{unsafe{raw::IsKeyDown(key)}}
pub fn IsKeyReleased(key: i32)->bool{unsafe{raw::IsKeyReleased(key)}}
pub fn IsKeyUp(key: i32)->bool{unsafe{raw::IsKeyUp(key)}}
pub fn GetKeyPressed()->i32{unsafe{raw::GetKeyPressed()}}
pub fn GetCharPressed()->i32{unsafe{raw::GetCharPressed()}}
pub fn GetKeyName(key: i32)->*const c_char{unsafe{raw::GetKeyName(key)}}
pub fn SetExitKey(key: i32){unsafe{raw::SetExitKey(key)}}
pub fn IsGamepadAvailable(gamepad: i32)->bool{unsafe{raw::IsGamepadAvailable(gamepad)}}
pub fn GetGamepadName(gamepad: i32)->*const c_char{unsafe{raw::GetGamepadName(gamepad)}}
pub fn IsGamepadButtonPressed(gamepad: i32, button: i32)->bool{unsafe{raw::IsGamepadButtonPressed(gamepad, button)}}
pub fn IsGamepadButtonDown(gamepad: i32, button: i32)->bool{unsafe{raw::IsGamepadButtonDown(gamepad, button)}}
pub fn IsGamepadButtonReleased(gamepad: i32, button: i32)->bool{unsafe{raw::IsGamepadButtonReleased(gamepad, button)}}
pub fn IsGamepadButtonUp(gamepad: i32, button: i32)->bool{unsafe{raw::IsGamepadButtonUp(gamepad, button)}}
pub fn GetGamepadButtonPressed()->i32{unsafe{raw::GetGamepadButtonPressed()}}
pub fn GetGamepadAxisCount(gamepad: i32)->i32{unsafe{raw::GetGamepadAxisCount(gamepad)}}
pub fn GetGamepadAxisMovement(gamepad: i32, axis: i32)->f32{unsafe{raw::GetGamepadAxisMovement(gamepad, axis)}}
pub fn SetGamepadMappings(mappings: impl AsRef<str>)->i32{unsafe{raw::SetGamepadMappings(CString::new(mappings.as_ref()).unwrap().as_ptr())}}
pub fn SetGamepadVibration(gamepad: i32, leftMotor: f32, rightMotor: f32, duration: f32){unsafe{raw::SetGamepadVibration(gamepad, leftMotor, rightMotor, duration)}}
pub fn IsMouseButtonPressed(button: i32)->bool{unsafe{raw::IsMouseButtonPressed(button)}}
pub fn IsMouseButtonDown(button: i32)->bool{unsafe{raw::IsMouseButtonDown(button)}}
pub fn IsMouseButtonReleased(button: i32)->bool{unsafe{raw::IsMouseButtonReleased(button)}}
pub fn IsMouseButtonUp(button: i32)->bool{unsafe{raw::IsMouseButtonUp(button)}}
pub fn GetMouseX()->i32{unsafe{raw::GetMouseX()}}
pub fn GetMouseY()->i32{unsafe{raw::GetMouseY()}}
pub fn GetMousePosition()->Vector2{unsafe{raw::GetMousePosition()}}
pub fn GetMouseDelta()->Vector2{unsafe{raw::GetMouseDelta()}}
pub fn SetMousePosition(x: i32, y: i32){unsafe{raw::SetMousePosition(x, y)}}
pub fn SetMouseOffset(offsetX: i32, offsetY: i32){unsafe{raw::SetMouseOffset(offsetX, offsetY)}}
pub fn SetMouseScale(scaleX: f32, scaleY: f32){unsafe{raw::SetMouseScale(scaleX, scaleY)}}
pub fn GetMouseWheelMove()->f32{unsafe{raw::GetMouseWheelMove()}}
pub fn GetMouseWheelMoveV()->Vector2{unsafe{raw::GetMouseWheelMoveV()}}
pub fn SetMouseCursor(cursor: i32){unsafe{raw::SetMouseCursor(cursor)}}
pub fn GetTouchX()->i32{unsafe{raw::GetTouchX()}}
pub fn GetTouchY()->i32{unsafe{raw::GetTouchY()}}
pub fn GetTouchPosition(index: i32)->Vector2{unsafe{raw::GetTouchPosition(index)}}
pub fn GetTouchPointId(index: i32)->i32{unsafe{raw::GetTouchPointId(index)}}
pub fn GetTouchPointCount()->i32{unsafe{raw::GetTouchPointCount()}}
pub fn SetGesturesEnabled(flags: u32){unsafe{raw::SetGesturesEnabled(flags)}}
pub fn IsGestureDetected(gesture: u32)->bool{unsafe{raw::IsGestureDetected(gesture)}}
pub fn GetGestureDetected()->i32{unsafe{raw::GetGestureDetected()}}
pub fn GetGestureHoldDuration()->f32{unsafe{raw::GetGestureHoldDuration()}}
pub fn GetGestureDragVector()->Vector2{unsafe{raw::GetGestureDragVector()}}
pub fn GetGestureDragAngle()->f32{unsafe{raw::GetGestureDragAngle()}}
pub fn GetGesturePinchVector()->Vector2{unsafe{raw::GetGesturePinchVector()}}
pub fn GetGesturePinchAngle()->f32{unsafe{raw::GetGesturePinchAngle()}}
pub fn UpdateCamera(camera: *mut Camera, mode: i32){unsafe{raw::UpdateCamera(camera, mode)}}
pub fn UpdateCameraPro(camera: *mut Camera, movement: Vector3, rotation: Vector3, zoom: f32){unsafe{raw::UpdateCameraPro(camera, movement, rotation, zoom)}}
pub fn SetShapesTexture(texture: Texture2D, rec: Rectangle){unsafe{raw::SetShapesTexture(texture, rec)}}
pub fn GetShapesTexture()->Texture2D{unsafe{raw::GetShapesTexture()}}
pub fn GetShapesTextureRectangle()->Rectangle{unsafe{raw::GetShapesTextureRectangle()}}
pub fn DrawPixel(posX: i32, posY: i32, color: Color){unsafe{raw::DrawPixel(posX, posY, color)}}
pub fn DrawPixelV(position: Vector2, color: Color){unsafe{raw::DrawPixelV(position, color)}}
pub fn DrawLine(startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color){unsafe{raw::DrawLine(startPosX, startPosY, endPosX, endPosY, color)}}
pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color){unsafe{raw::DrawLineV(startPos, endPos, color)}}
pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color){unsafe{raw::DrawLineEx(startPos, endPos, thick, color)}}
pub fn DrawLineStrip(points: *const Vector2, pointCount: i32, color: Color){unsafe{raw::DrawLineStrip(points, pointCount, color)}}
pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color){unsafe{raw::DrawLineBezier(startPos, endPos, thick, color)}}
pub fn DrawLineDashed(startPos: Vector2, endPos: Vector2, dashSize: i32, spaceSize: i32, color: Color){unsafe{raw::DrawLineDashed(startPos, endPos, dashSize, spaceSize, color)}}
pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color){unsafe{raw::DrawTriangle(v1, v2, v3, color)}}
pub fn DrawTriangleGradient(v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color){unsafe{raw::DrawTriangleGradient(v1, v2, v3, c1, c2, c3)}}
pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color){unsafe{raw::DrawTriangleLines(v1, v2, v3, color)}}
pub fn DrawTriangleFan(points: *const Vector2, pointCount: i32, color: Color){unsafe{raw::DrawTriangleFan(points, pointCount, color)}}
pub fn DrawTriangleStrip(points: *const Vector2, pointCount: i32, color: Color){unsafe{raw::DrawTriangleStrip(points, pointCount, color)}}
pub fn DrawRectangle(posX: i32, posY: i32, width: i32, height: i32, color: Color){unsafe{raw::DrawRectangle(posX, posY, width, height, color)}}
pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color){unsafe{raw::DrawRectangleV(position, size, color)}}
pub fn DrawRectangleRec(rec: Rectangle, color: Color){unsafe{raw::DrawRectangleRec(rec, color)}}
pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color){unsafe{raw::DrawRectanglePro(rec, origin, rotation, color)}}
pub fn DrawRectangleGradientV(posX: i32, posY: i32, width: i32, height: i32, top: Color, bottom: Color){unsafe{raw::DrawRectangleGradientV(posX, posY, width, height, top, bottom)}}
pub fn DrawRectangleGradientH(posX: i32, posY: i32, width: i32, height: i32, left: Color, right: Color){unsafe{raw::DrawRectangleGradientH(posX, posY, width, height, left, right)}}
pub fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color){unsafe{raw::DrawRectangleGradientEx(rec, col1, col2, col3, col4)}}
pub fn DrawRectangleLines(posX: i32, posY: i32, width: i32, height: i32, color: Color){unsafe{raw::DrawRectangleLines(posX, posY, width, height, color)}}
pub fn DrawRectangleLinesEx(rec: Rectangle, thick: f32, color: Color){unsafe{raw::DrawRectangleLinesEx(rec, thick, color)}}
pub fn DrawRectangleRounded(rec: Rectangle, roundness: f32, segments: i32, color: Color){unsafe{raw::DrawRectangleRounded(rec, roundness, segments, color)}}
pub fn DrawRectangleRoundedLines(rec: Rectangle, roundness: f32, segments: i32, color: Color){unsafe{raw::DrawRectangleRoundedLines(rec, roundness, segments, color)}}
pub fn DrawRectangleRoundedLinesEx(rec: Rectangle, roundness: f32, segments: i32, thick: f32, color: Color){unsafe{raw::DrawRectangleRoundedLinesEx(rec, roundness, segments, thick, color)}}
pub fn DrawPoly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color){unsafe{raw::DrawPoly(center, sides, radius, rotation, color)}}
pub fn DrawPolyLines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color){unsafe{raw::DrawPolyLines(center, sides, radius, rotation, color)}}
pub fn DrawPolyLinesEx(center: Vector2, sides: i32, radius: f32, rotation: f32, thick: f32, color: Color){unsafe{raw::DrawPolyLinesEx(center, sides, radius, rotation, thick, color)}}
pub fn DrawCircle(centerX: i32, centerY: i32, radius: f32, color: Color){unsafe{raw::DrawCircle(centerX, centerY, radius, color)}}
pub fn DrawCircleV(center: Vector2, radius: f32, color: Color){unsafe{raw::DrawCircleV(center, radius, color)}}
pub fn DrawCircleGradient(center: Vector2, radius: f32, inner: Color, outer: Color){unsafe{raw::DrawCircleGradient(center, radius, inner, outer)}}
pub fn DrawCircleSector(center: Vector2, radius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color){unsafe{raw::DrawCircleSector(center, radius, startAngle, endAngle, segments, color)}}
pub fn DrawCircleSectorLines(center: Vector2, radius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color){unsafe{raw::DrawCircleSectorLines(center, radius, startAngle, endAngle, segments, color)}}
pub fn DrawCircleLines(centerX: i32, centerY: i32, radius: f32, color: Color){unsafe{raw::DrawCircleLines(centerX, centerY, radius, color)}}
pub fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color){unsafe{raw::DrawCircleLinesV(center, radius, color)}}
pub fn DrawCircleLinesEx(center: Vector2, radius: f32, thick: f32, color: Color){unsafe{raw::DrawCircleLinesEx(center, radius, thick, color)}}
pub fn DrawEllipse(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color){unsafe{raw::DrawEllipse(centerX, centerY, radiusH, radiusV, color)}}
pub fn DrawEllipseV(center: Vector2, radiusH: f32, radiusV: f32, color: Color){unsafe{raw::DrawEllipseV(center, radiusH, radiusV, color)}}
pub fn DrawEllipseLines(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color){unsafe{raw::DrawEllipseLines(centerX, centerY, radiusH, radiusV, color)}}
pub fn DrawEllipseLinesV(center: Vector2, radiusH: f32, radiusV: f32, color: Color){unsafe{raw::DrawEllipseLinesV(center, radiusH, radiusV, color)}}
pub fn DrawRing(center: Vector2, innerRadius: f32, outerRadius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color){unsafe{raw::DrawRing(center, innerRadius, outerRadius, startAngle, endAngle, segments, color)}}
pub fn DrawRingLines(center: Vector2, innerRadius: f32, outerRadius: f32, startAngle: f32, endAngle: f32, segments: i32, color: Color){unsafe{raw::DrawRingLines(center, innerRadius, outerRadius, startAngle, endAngle, segments, color)}}
pub fn DrawSplineLinear(points: *const Vector2, pointCount: i32, thick: f32, color: Color){unsafe{raw::DrawSplineLinear(points, pointCount, thick, color)}}
pub fn DrawSplineBasis(points: *const Vector2, pointCount: i32, thick: f32, color: Color){unsafe{raw::DrawSplineBasis(points, pointCount, thick, color)}}
pub fn DrawSplineCatmullRom(points: *const Vector2, pointCount: i32, thick: f32, color: Color){unsafe{raw::DrawSplineCatmullRom(points, pointCount, thick, color)}}
pub fn DrawSplineBezierQuadratic(points: *const Vector2, pointCount: i32, thick: f32, color: Color){unsafe{raw::DrawSplineBezierQuadratic(points, pointCount, thick, color)}}
pub fn DrawSplineBezierCubic(points: *const Vector2, pointCount: i32, thick: f32, color: Color){unsafe{raw::DrawSplineBezierCubic(points, pointCount, thick, color)}}
pub fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color){unsafe{raw::DrawSplineSegmentLinear(p1, p2, thick, color)}}
pub fn DrawSplineSegmentBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color){unsafe{raw::DrawSplineSegmentBasis(p1, p2, p3, p4, thick, color)}}
pub fn DrawSplineSegmentCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color){unsafe{raw::DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thick, color)}}
pub fn DrawSplineSegmentBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, thick: f32, color: Color){unsafe{raw::DrawSplineSegmentBezierQuadratic(p1, c2, p3, thick, color)}}
pub fn DrawSplineSegmentBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, thick: f32, color: Color){unsafe{raw::DrawSplineSegmentBezierCubic(p1, c2, c3, p4, thick, color)}}
pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32)->Vector2{unsafe{raw::GetSplinePointLinear(startPos, endPos, t)}}
pub fn GetSplinePointBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32)->Vector2{unsafe{raw::GetSplinePointBasis(p1, p2, p3, p4, t)}}
pub fn GetSplinePointCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32)->Vector2{unsafe{raw::GetSplinePointCatmullRom(p1, p2, p3, p4, t)}}
pub fn GetSplinePointBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, t: f32)->Vector2{unsafe{raw::GetSplinePointBezierQuadratic(p1, c2, p3, t)}}
pub fn GetSplinePointBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, t: f32)->Vector2{unsafe{raw::GetSplinePointBezierCubic(p1, c2, c3, p4, t)}}
pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle)->bool{unsafe{raw::CheckCollisionRecs(rec1, rec2)}}
pub fn CheckCollisionCircles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32)->bool{unsafe{raw::CheckCollisionCircles(center1, radius1, center2, radius2)}}
pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle)->bool{unsafe{raw::CheckCollisionCircleRec(center, radius, rec)}}
pub fn CheckCollisionCircleLine(center: Vector2, radius: f32, p1: Vector2, p2: Vector2)->bool{unsafe{raw::CheckCollisionCircleLine(center, radius, p1, p2)}}
pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle)->bool{unsafe{raw::CheckCollisionPointRec(point, rec)}}
pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32)->bool{unsafe{raw::CheckCollisionPointCircle(point, center, radius)}}
pub fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2)->bool{unsafe{raw::CheckCollisionPointTriangle(point, p1, p2, p3)}}
pub fn CheckCollisionPointLine(point: Vector2, p1: Vector2, p2: Vector2, threshold: i32)->bool{unsafe{raw::CheckCollisionPointLine(point, p1, p2, threshold)}}
pub fn CheckCollisionPointPoly(point: Vector2, points: *const Vector2, pointCount: i32)->bool{unsafe{raw::CheckCollisionPointPoly(point, points, pointCount)}}
pub fn CheckCollisionLines(startPos1: Vector2, endPos1: Vector2, startPos2: Vector2, endPos2: Vector2, collisionPoint: *mut Vector2)->bool{unsafe{raw::CheckCollisionLines(startPos1, endPos1, startPos2, endPos2, collisionPoint)}}
pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle)->Rectangle{unsafe{raw::GetCollisionRec(rec1, rec2)}}
pub fn LoadImage(fileName: impl AsRef<str>)->Image{unsafe{raw::LoadImage(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadImageRaw(fileName: impl AsRef<str>, width: i32, height: i32, format: i32, headerSize: i32)->Image{unsafe{raw::LoadImageRaw(CString::new(fileName.as_ref()).unwrap().as_ptr(), width, height, format, headerSize)}}
pub fn LoadImageAnim(fileName: impl AsRef<str>, frames: *mut i32)->Image{unsafe{raw::LoadImageAnim(CString::new(fileName.as_ref()).unwrap().as_ptr(), frames)}}
pub fn LoadImageAnimFromMemory(fileType: impl AsRef<str>, fileData: *const c_uchar, dataSize: i32, frames: *mut i32)->Image{unsafe{raw::LoadImageAnimFromMemory(CString::new(fileType.as_ref()).unwrap().as_ptr(), fileData, dataSize, frames)}}
pub fn LoadImageFromMemory(fileType: impl AsRef<str>, fileData: *const c_uchar, dataSize: i32)->Image{unsafe{raw::LoadImageFromMemory(CString::new(fileType.as_ref()).unwrap().as_ptr(), fileData, dataSize)}}
pub fn LoadImageFromTexture(texture: Texture2D)->Image{unsafe{raw::LoadImageFromTexture(texture)}}
pub fn LoadImageFromScreen()->Image{unsafe{raw::LoadImageFromScreen()}}
pub fn IsImageValid(image: Image)->bool{unsafe{raw::IsImageValid(image)}}
pub fn UnloadImage(image: Image){unsafe{raw::UnloadImage(image)}}
pub fn ExportImage(image: Image, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportImage(image, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn ExportImageToMemory(image: Image, fileType: impl AsRef<str>, fileSize: *mut i32)->*mut c_uchar{unsafe{raw::ExportImageToMemory(image, CString::new(fileType.as_ref()).unwrap().as_ptr(), fileSize)}}
pub fn ExportImageAsCode(image: Image, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportImageAsCode(image, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn GenImageColor(width: i32, height: i32, color: Color)->Image{unsafe{raw::GenImageColor(width, height, color)}}
pub fn GenImageGradientLinear(width: i32, height: i32, direction: i32, start: Color, end: Color)->Image{unsafe{raw::GenImageGradientLinear(width, height, direction, start, end)}}
pub fn GenImageGradientRadial(width: i32, height: i32, density: f32, inner: Color, outer: Color)->Image{unsafe{raw::GenImageGradientRadial(width, height, density, inner, outer)}}
pub fn GenImageGradientSquare(width: i32, height: i32, density: f32, inner: Color, outer: Color)->Image{unsafe{raw::GenImageGradientSquare(width, height, density, inner, outer)}}
pub fn GenImageChecked(width: i32, height: i32, checksX: i32, checksY: i32, col1: Color, col2: Color)->Image{unsafe{raw::GenImageChecked(width, height, checksX, checksY, col1, col2)}}
pub fn GenImageWhiteNoise(width: i32, height: i32, factor: f32)->Image{unsafe{raw::GenImageWhiteNoise(width, height, factor)}}
pub fn GenImagePerlinNoise(width: i32, height: i32, offsetX: i32, offsetY: i32, scale: f32)->Image{unsafe{raw::GenImagePerlinNoise(width, height, offsetX, offsetY, scale)}}
pub fn GenImageCellular(width: i32, height: i32, tileSize: i32)->Image{unsafe{raw::GenImageCellular(width, height, tileSize)}}
pub fn GenImageText(width: i32, height: i32, text: impl AsRef<str>)->Image{unsafe{raw::GenImageText(width, height, CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn ImageCopy(image: Image)->Image{unsafe{raw::ImageCopy(image)}}
pub fn ImageFromImage(image: Image, rec: Rectangle)->Image{unsafe{raw::ImageFromImage(image, rec)}}
pub fn ImageFromChannel(image: Image, selectedChannel: i32)->Image{unsafe{raw::ImageFromChannel(image, selectedChannel)}}
pub fn ImageText(text: impl AsRef<str>, fontSize: i32, color: Color)->Image{unsafe{raw::ImageText(CString::new(text.as_ref()).unwrap().as_ptr(), fontSize, color)}}
pub fn ImageTextEx(font: Font, text: impl AsRef<str>, fontSize: f32, spacing: f32, tint: Color)->Image{unsafe{raw::ImageTextEx(font, CString::new(text.as_ref()).unwrap().as_ptr(), fontSize, spacing, tint)}}
pub fn ImageFormat(image: *mut Image, newFormat: i32){unsafe{raw::ImageFormat(image, newFormat)}}
pub fn ImageToPOT(image: *mut Image, fill: Color){unsafe{raw::ImageToPOT(image, fill)}}
pub fn ImageCrop(image: *mut Image, crop: Rectangle){unsafe{raw::ImageCrop(image, crop)}}
pub fn ImageAlphaCrop(image: *mut Image, threshold: f32){unsafe{raw::ImageAlphaCrop(image, threshold)}}
pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32){unsafe{raw::ImageAlphaClear(image, color, threshold)}}
pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image){unsafe{raw::ImageAlphaMask(image, alphaMask)}}
pub fn ImageAlphaPremultiply(image: *mut Image){unsafe{raw::ImageAlphaPremultiply(image)}}
pub fn ImageBlurGaussian(image: *mut Image, blurSize: i32){unsafe{raw::ImageBlurGaussian(image, blurSize)}}
pub fn ImageKernelConvolution(image: *mut Image, kernel: *const f32, kernelSize: i32){unsafe{raw::ImageKernelConvolution(image, kernel, kernelSize)}}
pub fn ImageResize(image: *mut Image, newWidth: i32, newHeight: i32){unsafe{raw::ImageResize(image, newWidth, newHeight)}}
pub fn ImageResizeNN(image: *mut Image, newWidth: i32, newHeight: i32){unsafe{raw::ImageResizeNN(image, newWidth, newHeight)}}
pub fn ImageResizeCanvas(image: *mut Image, newWidth: i32, newHeight: i32, offsetX: i32, offsetY: i32, fill: Color){unsafe{raw::ImageResizeCanvas(image, newWidth, newHeight, offsetX, offsetY, fill)}}
pub fn ImageMipmaps(image: *mut Image){unsafe{raw::ImageMipmaps(image)}}
pub fn ImageDither(image: *mut Image, rBpp: i32, gBpp: i32, bBpp: i32, aBpp: i32){unsafe{raw::ImageDither(image, rBpp, gBpp, bBpp, aBpp)}}
pub fn ImageFlipVertical(image: *mut Image){unsafe{raw::ImageFlipVertical(image)}}
pub fn ImageFlipHorizontal(image: *mut Image){unsafe{raw::ImageFlipHorizontal(image)}}
pub fn ImageRotate(image: *mut Image, degrees: i32){unsafe{raw::ImageRotate(image, degrees)}}
pub fn ImageRotateCW(image: *mut Image){unsafe{raw::ImageRotateCW(image)}}
pub fn ImageRotateCCW(image: *mut Image){unsafe{raw::ImageRotateCCW(image)}}
pub fn ImageColorTint(image: *mut Image, color: Color){unsafe{raw::ImageColorTint(image, color)}}
pub fn ImageColorInvert(image: *mut Image){unsafe{raw::ImageColorInvert(image)}}
pub fn ImageColorGrayscale(image: *mut Image){unsafe{raw::ImageColorGrayscale(image)}}
pub fn ImageColorContrast(image: *mut Image, contrast: i32){unsafe{raw::ImageColorContrast(image, contrast)}}
pub fn ImageColorBrightness(image: *mut Image, brightness: i32){unsafe{raw::ImageColorBrightness(image, brightness)}}
pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color){unsafe{raw::ImageColorReplace(image, color, replace)}}
pub fn LoadImageColors(image: Image)->*mut Color{unsafe{raw::LoadImageColors(image)}}
pub fn LoadImagePalette(image: Image, maxPaletteSize: i32, colorCount: *mut i32)->*mut Color{unsafe{raw::LoadImagePalette(image, maxPaletteSize, colorCount)}}
pub fn UnloadImageColors(colors: *mut Color){unsafe{raw::UnloadImageColors(colors)}}
pub fn UnloadImagePalette(colors: *mut Color){unsafe{raw::UnloadImagePalette(colors)}}
pub fn GetImageAlphaBorder(image: Image, threshold: f32)->Rectangle{unsafe{raw::GetImageAlphaBorder(image, threshold)}}
pub fn GetImageColor(image: Image, x: i32, y: i32)->Color{unsafe{raw::GetImageColor(image, x, y)}}
pub fn ImageClearBackground(dst: *mut Image, color: Color){unsafe{raw::ImageClearBackground(dst, color)}}
pub fn ImageDrawPixel(dst: *mut Image, posX: i32, posY: i32, color: Color){unsafe{raw::ImageDrawPixel(dst, posX, posY, color)}}
pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color){unsafe{raw::ImageDrawPixelV(dst, position, color)}}
pub fn ImageDrawLine(dst: *mut Image, startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color){unsafe{raw::ImageDrawLine(dst, startPosX, startPosY, endPosX, endPosY, color)}}
pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color){unsafe{raw::ImageDrawLineV(dst, start, end, color)}}
pub fn ImageDrawLineEx(dst: *mut Image, start: Vector2, end: Vector2, thick: i32, color: Color){unsafe{raw::ImageDrawLineEx(dst, start, end, thick, color)}}
pub fn ImageDrawLineStrip(dst: *mut Image, points: *const Vector2, pointCount: i32, color: Color){unsafe{raw::ImageDrawLineStrip(dst, points, pointCount, color)}}
pub fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color){unsafe{raw::ImageDrawTriangle(dst, v1, v2, v3, color)}}
pub fn ImageDrawTriangleGradient(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color){unsafe{raw::ImageDrawTriangleGradient(dst, v1, v2, v3, c1, c2, c3)}}
pub fn ImageDrawTriangleLines(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color){unsafe{raw::ImageDrawTriangleLines(dst, v1, v2, v3, color)}}
pub fn ImageDrawTriangleFan(dst: *mut Image, points: *const Vector2, pointCount: i32, color: Color){unsafe{raw::ImageDrawTriangleFan(dst, points, pointCount, color)}}
pub fn ImageDrawTriangleStrip(dst: *mut Image, points: *const Vector2, pointCount: i32, color: Color){unsafe{raw::ImageDrawTriangleStrip(dst, points, pointCount, color)}}
pub fn ImageDrawRectangle(dst: *mut Image, posX: i32, posY: i32, width: i32, height: i32, color: Color){unsafe{raw::ImageDrawRectangle(dst, posX, posY, width, height, color)}}
pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color){unsafe{raw::ImageDrawRectangleV(dst, position, size, color)}}
pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color){unsafe{raw::ImageDrawRectangleRec(dst, rec, color)}}
pub fn ImageDrawRectanglePro(dst: *mut Image, rec: Rectangle, origin: Vector2, rotation: f32, color: Color){unsafe{raw::ImageDrawRectanglePro(dst, rec, origin, rotation, color)}}
pub fn ImageDrawRectangleLines(dst: *mut Image, posX: i32, posY: i32, width: i32, height: i32, color: Color){unsafe{raw::ImageDrawRectangleLines(dst, posX, posY, width, height, color)}}
pub fn ImageDrawRectangleLinesEx(dst: *mut Image, rec: Rectangle, thick: i32, color: Color){unsafe{raw::ImageDrawRectangleLinesEx(dst, rec, thick, color)}}
pub fn ImageDrawRectangleGradientEx(dst: *mut Image, rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color){unsafe{raw::ImageDrawRectangleGradientEx(dst, rec, col1, col2, col3, col4)}}
pub fn ImageDrawCircle(dst: *mut Image, centerX: i32, centerY: i32, radius: i32, color: Color){unsafe{raw::ImageDrawCircle(dst, centerX, centerY, radius, color)}}
pub fn ImageDrawCircleV(dst: *mut Image, center: Vector2, radius: i32, color: Color){unsafe{raw::ImageDrawCircleV(dst, center, radius, color)}}
pub fn ImageDrawCircleLines(dst: *mut Image, centerX: i32, centerY: i32, radius: i32, color: Color){unsafe{raw::ImageDrawCircleLines(dst, centerX, centerY, radius, color)}}
pub fn ImageDrawCircleLinesV(dst: *mut Image, center: Vector2, radius: i32, color: Color){unsafe{raw::ImageDrawCircleLinesV(dst, center, radius, color)}}
pub fn ImageDrawCircleGradient(dst: *mut Image, center: Vector2, radius: f32, inner: Color, outer: Color){unsafe{raw::ImageDrawCircleGradient(dst, center, radius, inner, outer)}}
pub fn ImageDrawImage(dst: *mut Image, src: Image, posX: i32, posY: i32, tint: Color){unsafe{raw::ImageDrawImage(dst, src, posX, posY, tint)}}
pub fn ImageDrawImageEx(dst: *mut Image, src: Image, position: Vector2, rotation: f32, scale: f32, tint: Color){unsafe{raw::ImageDrawImageEx(dst, src, position, rotation, scale, tint)}}
pub fn ImageDrawImageRec(dst: *mut Image, src: Image, srcRec: Rectangle, position: Vector2, tint: Color){unsafe{raw::ImageDrawImageRec(dst, src, srcRec, position, tint)}}
pub fn ImageDrawImagePro(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle, origin: Vector2, rotation: f32, tint: Color){unsafe{raw::ImageDrawImagePro(dst, src, srcRec, dstRec, origin, rotation, tint)}}
pub fn ImageDrawText(dst: *mut Image, text: impl AsRef<str>, posX: i32, posY: i32, fontSize: i32, color: Color){unsafe{raw::ImageDrawText(dst, CString::new(text.as_ref()).unwrap().as_ptr(), posX, posY, fontSize, color)}}
pub fn ImageDrawTextEx(dst: *mut Image, font: Font, text: impl AsRef<str>, position: Vector2, fontSize: f32, spacing: f32, tint: Color){unsafe{raw::ImageDrawTextEx(dst, font, CString::new(text.as_ref()).unwrap().as_ptr(), position, fontSize, spacing, tint)}}
pub fn ImageDrawTextPro(dst: *mut Image, font: Font, text: impl AsRef<str>, position: Vector2, origin: Vector2, rotation: f32, fontSize: f32, spacing: f32, tint: Color){unsafe{raw::ImageDrawTextPro(dst, font, CString::new(text.as_ref()).unwrap().as_ptr(), position, origin, rotation, fontSize, spacing, tint)}}
pub fn LoadTexture(fileName: impl AsRef<str>)->Texture2D{unsafe{raw::LoadTexture(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadTextureFromImage(image: Image)->Texture2D{unsafe{raw::LoadTextureFromImage(image)}}
pub fn LoadTextureCubemap(image: Image, layout: i32)->TextureCubemap{unsafe{raw::LoadTextureCubemap(image, layout)}}
pub fn LoadRenderTexture(width: i32, height: i32)->RenderTexture2D{unsafe{raw::LoadRenderTexture(width, height)}}
pub fn IsTextureValid(texture: Texture2D)->bool{unsafe{raw::IsTextureValid(texture)}}
pub fn UnloadTexture(texture: Texture2D){unsafe{raw::UnloadTexture(texture)}}
pub fn IsRenderTextureValid(target: RenderTexture2D)->bool{unsafe{raw::IsRenderTextureValid(target)}}
pub fn UnloadRenderTexture(target: RenderTexture2D){unsafe{raw::UnloadRenderTexture(target)}}
pub fn UpdateTexture(texture: Texture2D, pixels: *const c_void){unsafe{raw::UpdateTexture(texture, pixels)}}
pub fn UpdateTextureRec(texture: Texture2D, rec: Rectangle, pixels: *const c_void){unsafe{raw::UpdateTextureRec(texture, rec, pixels)}}
pub fn GenTextureMipmaps(texture: *mut Texture2D){unsafe{raw::GenTextureMipmaps(texture)}}
pub fn SetTextureFilter(texture: Texture2D, filter: i32){unsafe{raw::SetTextureFilter(texture, filter)}}
pub fn SetTextureWrap(texture: Texture2D, wrap: i32){unsafe{raw::SetTextureWrap(texture, wrap)}}
pub fn DrawTexture(texture: Texture2D, posX: i32, posY: i32, tint: Color){unsafe{raw::DrawTexture(texture, posX, posY, tint)}}
pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color){unsafe{raw::DrawTextureV(texture, position, tint)}}
pub fn DrawTextureEx(texture: Texture2D, position: Vector2, rotation: f32, scale: f32, tint: Color){unsafe{raw::DrawTextureEx(texture, position, rotation, scale, tint)}}
pub fn DrawTextureRec(texture: Texture2D, rec: Rectangle, position: Vector2, tint: Color){unsafe{raw::DrawTextureRec(texture, rec, position, tint)}}
pub fn DrawTexturePro(texture: Texture2D, srcrec: Rectangle, dstrec: Rectangle, origin: Vector2, rotation: f32, tint: Color){unsafe{raw::DrawTexturePro(texture, srcrec, dstrec, origin, rotation, tint)}}
pub fn DrawTextureNPatch(texture: Texture2D, nPatchInfo: NPatchInfo, dstrec: Rectangle, origin: Vector2, rotation: f32, tint: Color){unsafe{raw::DrawTextureNPatch(texture, nPatchInfo, dstrec, origin, rotation, tint)}}
pub fn ColorIsEqual(col1: Color, col2: Color)->bool{unsafe{raw::ColorIsEqual(col1, col2)}}
pub fn Fade(color: Color, alpha: f32)->Color{unsafe{raw::Fade(color, alpha)}}
pub fn ColorToInt(color: Color)->i32{unsafe{raw::ColorToInt(color)}}
pub fn ColorNormalize(color: Color)->Vector4{unsafe{raw::ColorNormalize(color)}}
pub fn ColorFromNormalized(normalized: Vector4)->Color{unsafe{raw::ColorFromNormalized(normalized)}}
pub fn ColorToHSV(color: Color)->Vector3{unsafe{raw::ColorToHSV(color)}}
pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32)->Color{unsafe{raw::ColorFromHSV(hue, saturation, value)}}
pub fn ColorTint(color: Color, tint: Color)->Color{unsafe{raw::ColorTint(color, tint)}}
pub fn ColorBrightness(color: Color, factor: f32)->Color{unsafe{raw::ColorBrightness(color, factor)}}
pub fn ColorContrast(color: Color, contrast: f32)->Color{unsafe{raw::ColorContrast(color, contrast)}}
pub fn ColorAlpha(color: Color, alpha: f32)->Color{unsafe{raw::ColorAlpha(color, alpha)}}
pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color)->Color{unsafe{raw::ColorAlphaBlend(dst, src, tint)}}
pub fn ColorLerp(color1: Color, color2: Color, factor: f32)->Color{unsafe{raw::ColorLerp(color1, color2, factor)}}
pub fn GetColor(hexValue: u32)->Color{unsafe{raw::GetColor(hexValue)}}
pub fn GetPixelColor(srcPtr: *const c_void, format: i32)->Color{unsafe{raw::GetPixelColor(srcPtr, format)}}
pub fn SetPixelColor(dstPtr: *mut c_void, color: Color, format: i32){unsafe{raw::SetPixelColor(dstPtr, color, format)}}
pub fn GetPixelDataSize(width: i32, height: i32, format: i32)->i32{unsafe{raw::GetPixelDataSize(width, height, format)}}
pub fn GetFontDefault()->Font{unsafe{raw::GetFontDefault()}}
pub fn LoadFont(fileName: impl AsRef<str>)->Font{unsafe{raw::LoadFont(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadFontEx(fileName: impl AsRef<str>, fontSize: i32, codepoints: *const i32, codepointCount: i32)->Font{unsafe{raw::LoadFontEx(CString::new(fileName.as_ref()).unwrap().as_ptr(), fontSize, codepoints, codepointCount)}}
pub fn LoadFontFromImage(image: Image, key: Color, firstChar: i32)->Font{unsafe{raw::LoadFontFromImage(image, key, firstChar)}}
pub fn LoadFontFromMemory(fileType: impl AsRef<str>, fileData: *const c_uchar, dataSize: i32, fontSize: i32, codepoints: *const i32, codepointCount: i32)->Font{unsafe{raw::LoadFontFromMemory(CString::new(fileType.as_ref()).unwrap().as_ptr(), fileData, dataSize, fontSize, codepoints, codepointCount)}}
pub fn IsFontValid(font: Font)->bool{unsafe{raw::IsFontValid(font)}}
pub fn LoadFontData(fileData: *const c_uchar, dataSize: i32, fontSize: i32, codepoints: *const i32, codepointCount: i32, r#type: i32, glyphCount: *mut i32)->*mut GlyphInfo{unsafe{raw::LoadFontData(fileData, dataSize, fontSize, codepoints, codepointCount, r#type, glyphCount)}}
pub fn GenImageFontAtlas(glyphs: *const GlyphInfo, glyphRecs: *mut Rectangle, glyphCount: i32, fontSize: i32, padding: i32, packMethod: i32)->Image{unsafe{raw::GenImageFontAtlas(glyphs, glyphRecs, glyphCount, fontSize, padding, packMethod)}}
pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: i32){unsafe{raw::UnloadFontData(glyphs, glyphCount)}}
pub fn UnloadFont(font: Font){unsafe{raw::UnloadFont(font)}}
pub fn ExportFontAsCode(font: Font, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportFontAsCode(font, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn DrawFPS(posX: i32, posY: i32){unsafe{raw::DrawFPS(posX, posY)}}
pub fn DrawText(text: impl AsRef<str>, posX: i32, posY: i32, fontSize: i32, color: Color){unsafe{raw::DrawText(CString::new(text.as_ref()).unwrap().as_ptr(), posX, posY, fontSize, color)}}
pub fn DrawTextEx(font: Font, text: impl AsRef<str>, position: Vector2, fontSize: f32, spacing: f32, tint: Color){unsafe{raw::DrawTextEx(font, CString::new(text.as_ref()).unwrap().as_ptr(), position, fontSize, spacing, tint)}}
pub fn DrawTextPro(font: Font, text: impl AsRef<str>, position: Vector2, origin: Vector2, rotation: f32, fontSize: f32, spacing: f32, tint: Color){unsafe{raw::DrawTextPro(font, CString::new(text.as_ref()).unwrap().as_ptr(), position, origin, rotation, fontSize, spacing, tint)}}
pub fn DrawTextCodepoint(font: Font, codepoint: i32, position: Vector2, fontSize: f32, tint: Color){unsafe{raw::DrawTextCodepoint(font, codepoint, position, fontSize, tint)}}
pub fn DrawTextCodepoints(font: Font, codepoints: *const i32, codepointCount: i32, position: Vector2, fontSize: f32, spacing: f32, tint: Color){unsafe{raw::DrawTextCodepoints(font, codepoints, codepointCount, position, fontSize, spacing, tint)}}
pub fn SetTextLineSpacing(spacing: i32){unsafe{raw::SetTextLineSpacing(spacing)}}
pub fn MeasureText(text: impl AsRef<str>, fontSize: i32)->i32{unsafe{raw::MeasureText(CString::new(text.as_ref()).unwrap().as_ptr(), fontSize)}}
pub fn MeasureTextEx(font: Font, text: impl AsRef<str>, fontSize: f32, spacing: f32)->Vector2{unsafe{raw::MeasureTextEx(font, CString::new(text.as_ref()).unwrap().as_ptr(), fontSize, spacing)}}
pub fn MeasureTextCodepoints(font: Font, codepoints: *const i32, length: i32, fontSize: f32, spacing: f32)->Vector2{unsafe{raw::MeasureTextCodepoints(font, codepoints, length, fontSize, spacing)}}
pub fn GetGlyphIndex(font: Font, codepoint: i32)->i32{unsafe{raw::GetGlyphIndex(font, codepoint)}}
pub fn GetGlyphInfo(font: Font, codepoint: i32)->GlyphInfo{unsafe{raw::GetGlyphInfo(font, codepoint)}}
pub fn GetGlyphAtlasRec(font: Font, codepoint: i32)->Rectangle{unsafe{raw::GetGlyphAtlasRec(font, codepoint)}}
pub fn LoadUTF8(codepoints: *const i32, length: i32)->*mut c_char{unsafe{raw::LoadUTF8(codepoints, length)}}
pub fn UnloadUTF8(text: *mut c_char){unsafe{raw::UnloadUTF8(text)}}
pub fn LoadCodepoints(text: impl AsRef<str>, count: *mut i32)->*mut i32{unsafe{raw::LoadCodepoints(CString::new(text.as_ref()).unwrap().as_ptr(), count)}}
pub fn UnloadCodepoints(codepoints: *mut i32){unsafe{raw::UnloadCodepoints(codepoints)}}
pub fn GetCodepointCount(text: impl AsRef<str>)->i32{unsafe{raw::GetCodepointCount(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn GetCodepoint(text: impl AsRef<str>, codepointSize: *mut i32)->i32{unsafe{raw::GetCodepoint(CString::new(text.as_ref()).unwrap().as_ptr(), codepointSize)}}
pub fn GetCodepointNext(text: impl AsRef<str>, codepointSize: *mut i32)->i32{unsafe{raw::GetCodepointNext(CString::new(text.as_ref()).unwrap().as_ptr(), codepointSize)}}
pub fn GetCodepointPrevious(text: impl AsRef<str>, codepointSize: *mut i32)->i32{unsafe{raw::GetCodepointPrevious(CString::new(text.as_ref()).unwrap().as_ptr(), codepointSize)}}
pub fn CodepointToUTF8(codepoint: i32, utf8Size: *mut i32)->*const c_char{unsafe{raw::CodepointToUTF8(codepoint, utf8Size)}}
pub fn LoadTextLines(text: impl AsRef<str>, count: *mut i32)->*mut c_char{unsafe{raw::LoadTextLines(CString::new(text.as_ref()).unwrap().as_ptr(), count)}}
pub fn UnloadTextLines(text: *mut c_char, lineCount: i32){unsafe{raw::UnloadTextLines(text, lineCount)}}
pub fn TextCopy(dst: *mut c_char, src: impl AsRef<str>)->i32{unsafe{raw::TextCopy(dst, CString::new(src.as_ref()).unwrap().as_ptr())}}
pub fn TextIsEqual(text1: impl AsRef<str>, text2: impl AsRef<str>)->bool{unsafe{raw::TextIsEqual(CString::new(text1.as_ref()).unwrap().as_ptr(), CString::new(text2.as_ref()).unwrap().as_ptr())}}
pub fn TextLength(text: impl AsRef<str>)->u32{unsafe{raw::TextLength(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextFormat(text: impl AsRef<str>)->*const c_char{unsafe{raw::TextFormat(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextSubtext(text: impl AsRef<str>, position: i32, length: i32)->*const c_char{unsafe{raw::TextSubtext(CString::new(text.as_ref()).unwrap().as_ptr(), position, length)}}
pub fn TextRemoveSpaces(text: impl AsRef<str>)->*const c_char{unsafe{raw::TextRemoveSpaces(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn GetTextBetween(text: impl AsRef<str>, begin: impl AsRef<str>, end: impl AsRef<str>)->*mut c_char{unsafe{raw::GetTextBetween(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(begin.as_ref()).unwrap().as_ptr(), CString::new(end.as_ref()).unwrap().as_ptr())}}
pub fn TextReplace(text: impl AsRef<str>, search: impl AsRef<str>, replacement: impl AsRef<str>)->*mut c_char{unsafe{raw::TextReplace(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(search.as_ref()).unwrap().as_ptr(), CString::new(replacement.as_ref()).unwrap().as_ptr())}}
pub fn TextReplaceAlloc(text: impl AsRef<str>, search: impl AsRef<str>, replacement: impl AsRef<str>)->*mut c_char{unsafe{raw::TextReplaceAlloc(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(search.as_ref()).unwrap().as_ptr(), CString::new(replacement.as_ref()).unwrap().as_ptr())}}
pub fn TextReplaceBetween(text: impl AsRef<str>, begin: impl AsRef<str>, end: impl AsRef<str>, replacement: impl AsRef<str>)->*mut c_char{unsafe{raw::TextReplaceBetween(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(begin.as_ref()).unwrap().as_ptr(), CString::new(end.as_ref()).unwrap().as_ptr(), CString::new(replacement.as_ref()).unwrap().as_ptr())}}
pub fn TextReplaceBetweenAlloc(text: impl AsRef<str>, begin: impl AsRef<str>, end: impl AsRef<str>, replacement: impl AsRef<str>)->*mut c_char{unsafe{raw::TextReplaceBetweenAlloc(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(begin.as_ref()).unwrap().as_ptr(), CString::new(end.as_ref()).unwrap().as_ptr(), CString::new(replacement.as_ref()).unwrap().as_ptr())}}
pub fn TextInsert(text: impl AsRef<str>, insert: impl AsRef<str>, position: i32)->*mut c_char{unsafe{raw::TextInsert(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(insert.as_ref()).unwrap().as_ptr(), position)}}
pub fn TextInsertAlloc(text: impl AsRef<str>, insert: impl AsRef<str>, position: i32)->*mut c_char{unsafe{raw::TextInsertAlloc(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(insert.as_ref()).unwrap().as_ptr(), position)}}
pub fn TextJoin(textList: *mut c_char, count: i32, delimiter: impl AsRef<str>)->*mut c_char{unsafe{raw::TextJoin(textList, count, CString::new(delimiter.as_ref()).unwrap().as_ptr())}}
pub fn TextSplit(text: impl AsRef<str>, delimiter: i8, count: *mut i32)->*mut c_char{unsafe{raw::TextSplit(CString::new(text.as_ref()).unwrap().as_ptr(), delimiter, count)}}
pub fn TextAppend(text: *mut c_char, append: impl AsRef<str>, position: *mut i32){unsafe{raw::TextAppend(text, CString::new(append.as_ref()).unwrap().as_ptr(), position)}}
pub fn TextFindIndex(text: impl AsRef<str>, search: impl AsRef<str>)->i32{unsafe{raw::TextFindIndex(CString::new(text.as_ref()).unwrap().as_ptr(), CString::new(search.as_ref()).unwrap().as_ptr())}}
pub fn TextToUpper(text: impl AsRef<str>)->*mut c_char{unsafe{raw::TextToUpper(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextToLower(text: impl AsRef<str>)->*mut c_char{unsafe{raw::TextToLower(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextToPascal(text: impl AsRef<str>)->*mut c_char{unsafe{raw::TextToPascal(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextToSnake(text: impl AsRef<str>)->*mut c_char{unsafe{raw::TextToSnake(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextToCamel(text: impl AsRef<str>)->*mut c_char{unsafe{raw::TextToCamel(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextToInteger(text: impl AsRef<str>)->i32{unsafe{raw::TextToInteger(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn TextToFloat(text: impl AsRef<str>)->f32{unsafe{raw::TextToFloat(CString::new(text.as_ref()).unwrap().as_ptr())}}
pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color){unsafe{raw::DrawLine3D(startPos, endPos, color)}}
pub fn DrawPoint3D(position: Vector3, color: Color){unsafe{raw::DrawPoint3D(position, color)}}
pub fn DrawCircle3D(center: Vector3, radius: f32, rotationAxis: Vector3, rotationAngle: f32, color: Color){unsafe{raw::DrawCircle3D(center, radius, rotationAxis, rotationAngle, color)}}
pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color){unsafe{raw::DrawTriangle3D(v1, v2, v3, color)}}
pub fn DrawTriangleStrip3D(points: *const Vector3, pointCount: i32, color: Color){unsafe{raw::DrawTriangleStrip3D(points, pointCount, color)}}
pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color){unsafe{raw::DrawCube(position, width, height, length, color)}}
pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color){unsafe{raw::DrawCubeV(position, size, color)}}
pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color){unsafe{raw::DrawCubeWires(position, width, height, length, color)}}
pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color){unsafe{raw::DrawCubeWiresV(position, size, color)}}
pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color){unsafe{raw::DrawSphere(centerPos, radius, color)}}
pub fn DrawSphereEx(centerPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color){unsafe{raw::DrawSphereEx(centerPos, radius, rings, slices, color)}}
pub fn DrawSphereWires(centerPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color){unsafe{raw::DrawSphereWires(centerPos, radius, rings, slices, color)}}
pub fn DrawCylinder(position: Vector3, radiusTop: f32, radiusBottom: f32, height: f32, sides: i32, color: Color){unsafe{raw::DrawCylinder(position, radiusTop, radiusBottom, height, sides, color)}}
pub fn DrawCylinderEx(startPos: Vector3, endPos: Vector3, startRadius: f32, endRadius: f32, sides: i32, color: Color){unsafe{raw::DrawCylinderEx(startPos, endPos, startRadius, endRadius, sides, color)}}
pub fn DrawCylinderWires(position: Vector3, radiusTop: f32, radiusBottom: f32, height: f32, sides: i32, color: Color){unsafe{raw::DrawCylinderWires(position, radiusTop, radiusBottom, height, sides, color)}}
pub fn DrawCylinderWiresEx(startPos: Vector3, endPos: Vector3, startRadius: f32, endRadius: f32, sides: i32, color: Color){unsafe{raw::DrawCylinderWiresEx(startPos, endPos, startRadius, endRadius, sides, color)}}
pub fn DrawCapsule(startPos: Vector3, endPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color){unsafe{raw::DrawCapsule(startPos, endPos, radius, rings, slices, color)}}
pub fn DrawCapsuleWires(startPos: Vector3, endPos: Vector3, radius: f32, rings: i32, slices: i32, color: Color){unsafe{raw::DrawCapsuleWires(startPos, endPos, radius, rings, slices, color)}}
pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color){unsafe{raw::DrawPlane(centerPos, size, color)}}
pub fn DrawRay(ray: Ray, color: Color){unsafe{raw::DrawRay(ray, color)}}
pub fn DrawGrid(slices: i32, spacing: f32){unsafe{raw::DrawGrid(slices, spacing)}}
pub fn LoadModel(fileName: impl AsRef<str>)->Model{unsafe{raw::LoadModel(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadModelFromMesh(mesh: Mesh)->Model{unsafe{raw::LoadModelFromMesh(mesh)}}
pub fn IsModelValid(model: Model)->bool{unsafe{raw::IsModelValid(model)}}
pub fn UnloadModel(model: Model){unsafe{raw::UnloadModel(model)}}
pub fn GetModelBoundingBox(model: Model)->BoundingBox{unsafe{raw::GetModelBoundingBox(model)}}
pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color){unsafe{raw::DrawModel(model, position, scale, tint)}}
pub fn DrawModelEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: f32, scale: Vector3, tint: Color){unsafe{raw::DrawModelEx(model, position, rotationAxis, rotationAngle, scale, tint)}}
pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color){unsafe{raw::DrawModelWires(model, position, scale, tint)}}
pub fn DrawModelWiresEx(model: Model, position: Vector3, rotationAxis: Vector3, rotationAngle: f32, scale: Vector3, tint: Color){unsafe{raw::DrawModelWiresEx(model, position, rotationAxis, rotationAngle, scale, tint)}}
pub fn DrawBoundingBox(r#box: BoundingBox, color: Color){unsafe{raw::DrawBoundingBox(r#box, color)}}
pub fn DrawBillboard(camera: Camera, texture: Texture2D, position: Vector3, scale: f32, tint: Color){unsafe{raw::DrawBillboard(camera, texture, position, scale, tint)}}
pub fn DrawBillboardRec(camera: Camera, texture: Texture2D, rec: Rectangle, position: Vector3, size: Vector2, tint: Color){unsafe{raw::DrawBillboardRec(camera, texture, rec, position, size, tint)}}
pub fn DrawBillboardPro(camera: Camera, texture: Texture2D, rec: Rectangle, position: Vector3, up: Vector3, size: Vector2, origin: Vector2, rotation: f32, tint: Color){unsafe{raw::DrawBillboardPro(camera, texture, rec, position, up, size, origin, rotation, tint)}}
pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool){unsafe{raw::UploadMesh(mesh, dynamic)}}
pub fn UpdateMeshBuffer(mesh: Mesh, index: i32, data: *const c_void, dataSize: i32, offset: i32){unsafe{raw::UpdateMeshBuffer(mesh, index, data, dataSize, offset)}}
pub fn UnloadMesh(mesh: Mesh){unsafe{raw::UnloadMesh(mesh)}}
pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix){unsafe{raw::DrawMesh(mesh, material, transform)}}
pub fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: *const Matrix, instances: i32){unsafe{raw::DrawMeshInstanced(mesh, material, transforms, instances)}}
pub fn GetMeshBoundingBox(mesh: Mesh)->BoundingBox{unsafe{raw::GetMeshBoundingBox(mesh)}}
pub fn GenMeshTangents(mesh: *mut Mesh){unsafe{raw::GenMeshTangents(mesh)}}
pub fn ExportMesh(mesh: Mesh, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportMesh(mesh, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn ExportMeshAsCode(mesh: Mesh, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportMeshAsCode(mesh, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn GenMeshPoly(sides: i32, radius: f32)->Mesh{unsafe{raw::GenMeshPoly(sides, radius)}}
pub fn GenMeshPlane(width: f32, length: f32, resX: i32, resZ: i32)->Mesh{unsafe{raw::GenMeshPlane(width, length, resX, resZ)}}
pub fn GenMeshCube(width: f32, height: f32, length: f32)->Mesh{unsafe{raw::GenMeshCube(width, height, length)}}
pub fn GenMeshSphere(radius: f32, rings: i32, slices: i32)->Mesh{unsafe{raw::GenMeshSphere(radius, rings, slices)}}
pub fn GenMeshHemiSphere(radius: f32, rings: i32, slices: i32)->Mesh{unsafe{raw::GenMeshHemiSphere(radius, rings, slices)}}
pub fn GenMeshCylinder(radius: f32, height: f32, slices: i32)->Mesh{unsafe{raw::GenMeshCylinder(radius, height, slices)}}
pub fn GenMeshCone(radius: f32, height: f32, slices: i32)->Mesh{unsafe{raw::GenMeshCone(radius, height, slices)}}
pub fn GenMeshTorus(radius: f32, size: f32, radSeg: i32, sides: i32)->Mesh{unsafe{raw::GenMeshTorus(radius, size, radSeg, sides)}}
pub fn GenMeshKnot(radius: f32, size: f32, radSeg: i32, sides: i32)->Mesh{unsafe{raw::GenMeshKnot(radius, size, radSeg, sides)}}
pub fn GenMeshHeightmap(heightmap: Image, size: Vector3)->Mesh{unsafe{raw::GenMeshHeightmap(heightmap, size)}}
pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3)->Mesh{unsafe{raw::GenMeshCubicmap(cubicmap, cubeSize)}}
pub fn LoadMaterials(fileName: impl AsRef<str>, materialCount: *mut i32)->*mut Material{unsafe{raw::LoadMaterials(CString::new(fileName.as_ref()).unwrap().as_ptr(), materialCount)}}
pub fn LoadMaterialDefault()->Material{unsafe{raw::LoadMaterialDefault()}}
pub fn IsMaterialValid(material: Material)->bool{unsafe{raw::IsMaterialValid(material)}}
pub fn UnloadMaterial(material: Material){unsafe{raw::UnloadMaterial(material)}}
pub fn SetMaterialTexture(material: *mut Material, mapType: i32, texture: Texture2D){unsafe{raw::SetMaterialTexture(material, mapType, texture)}}
pub fn SetModelMeshMaterial(model: *mut Model, meshId: i32, materialId: i32){unsafe{raw::SetModelMeshMaterial(model, meshId, materialId)}}
pub fn LoadModelAnimations(fileName: impl AsRef<str>, animCount: *mut i32)->*mut ModelAnimation{unsafe{raw::LoadModelAnimations(CString::new(fileName.as_ref()).unwrap().as_ptr(), animCount)}}
pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: f32){unsafe{raw::UpdateModelAnimation(model, anim, frame)}}
pub fn UpdateModelAnimationEx(model: Model, animA: ModelAnimation, frameA: f32, animB: ModelAnimation, frameB: f32, blend: f32){unsafe{raw::UpdateModelAnimationEx(model, animA, frameA, animB, frameB, blend)}}
pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: i32){unsafe{raw::UnloadModelAnimations(animations, animCount)}}
pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation)->bool{unsafe{raw::IsModelAnimationValid(model, anim)}}
pub fn CheckCollisionSpheres(center1: Vector3, radius1: f32, center2: Vector3, radius2: f32)->bool{unsafe{raw::CheckCollisionSpheres(center1, radius1, center2, radius2)}}
pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox)->bool{unsafe{raw::CheckCollisionBoxes(box1, box2)}}
pub fn CheckCollisionBoxSphere(r#box: BoundingBox, center: Vector3, radius: f32)->bool{unsafe{raw::CheckCollisionBoxSphere(r#box, center, radius)}}
pub fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: f32)->RayCollision{unsafe{raw::GetRayCollisionSphere(ray, center, radius)}}
pub fn GetRayCollisionBox(ray: Ray, r#box: BoundingBox)->RayCollision{unsafe{raw::GetRayCollisionBox(ray, r#box)}}
pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix)->RayCollision{unsafe{raw::GetRayCollisionMesh(ray, mesh, transform)}}
pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)->RayCollision{unsafe{raw::GetRayCollisionTriangle(ray, p1, p2, p3)}}
pub fn GetRayCollisionQuad(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3)->RayCollision{unsafe{raw::GetRayCollisionQuad(ray, p1, p2, p3, p4)}}
pub fn InitAudioDevice(){unsafe{raw::InitAudioDevice()}}
pub fn CloseAudioDevice(){unsafe{raw::CloseAudioDevice()}}
pub fn IsAudioDeviceReady()->bool{unsafe{raw::IsAudioDeviceReady()}}
pub fn SetMasterVolume(volume: f32){unsafe{raw::SetMasterVolume(volume)}}
pub fn GetMasterVolume()->f32{unsafe{raw::GetMasterVolume()}}
pub fn LoadWave(fileName: impl AsRef<str>)->Wave{unsafe{raw::LoadWave(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadWaveFromMemory(fileType: impl AsRef<str>, fileData: *const c_uchar, dataSize: i32)->Wave{unsafe{raw::LoadWaveFromMemory(CString::new(fileType.as_ref()).unwrap().as_ptr(), fileData, dataSize)}}
pub fn IsWaveValid(wave: Wave)->bool{unsafe{raw::IsWaveValid(wave)}}
pub fn LoadSound(fileName: impl AsRef<str>)->Sound{unsafe{raw::LoadSound(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadSoundFromWave(wave: Wave)->Sound{unsafe{raw::LoadSoundFromWave(wave)}}
pub fn LoadSoundAlias(source: Sound)->Sound{unsafe{raw::LoadSoundAlias(source)}}
pub fn IsSoundValid(sound: Sound)->bool{unsafe{raw::IsSoundValid(sound)}}
pub fn UpdateSound(sound: Sound, data: *const c_void, frameCount: i32){unsafe{raw::UpdateSound(sound, data, frameCount)}}
pub fn UnloadWave(wave: Wave){unsafe{raw::UnloadWave(wave)}}
pub fn UnloadSound(sound: Sound){unsafe{raw::UnloadSound(sound)}}
pub fn UnloadSoundAlias(alias: Sound){unsafe{raw::UnloadSoundAlias(alias)}}
pub fn ExportWave(wave: Wave, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportWave(wave, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn ExportWaveAsCode(wave: Wave, fileName: impl AsRef<str>)->bool{unsafe{raw::ExportWaveAsCode(wave, CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn PlaySound(sound: Sound){unsafe{raw::PlaySound(sound)}}
pub fn StopSound(sound: Sound){unsafe{raw::StopSound(sound)}}
pub fn PauseSound(sound: Sound){unsafe{raw::PauseSound(sound)}}
pub fn ResumeSound(sound: Sound){unsafe{raw::ResumeSound(sound)}}
pub fn IsSoundPlaying(sound: Sound)->bool{unsafe{raw::IsSoundPlaying(sound)}}
pub fn SetSoundVolume(sound: Sound, volume: f32){unsafe{raw::SetSoundVolume(sound, volume)}}
pub fn SetSoundPitch(sound: Sound, pitch: f32){unsafe{raw::SetSoundPitch(sound, pitch)}}
pub fn SetSoundPan(sound: Sound, pan: f32){unsafe{raw::SetSoundPan(sound, pan)}}
pub fn WaveCopy(wave: Wave)->Wave{unsafe{raw::WaveCopy(wave)}}
pub fn WaveCrop(wave: *mut Wave, initFrame: i32, finalFrame: i32){unsafe{raw::WaveCrop(wave, initFrame, finalFrame)}}
pub fn WaveFormat(wave: *mut Wave, sampleRate: i32, sampleSize: i32, channels: i32){unsafe{raw::WaveFormat(wave, sampleRate, sampleSize, channels)}}
pub fn LoadWaveSamples(wave: Wave)->*mut f32{unsafe{raw::LoadWaveSamples(wave)}}
pub fn UnloadWaveSamples(samples: *mut f32){unsafe{raw::UnloadWaveSamples(samples)}}
pub fn LoadMusicStream(fileName: impl AsRef<str>)->Music{unsafe{raw::LoadMusicStream(CString::new(fileName.as_ref()).unwrap().as_ptr())}}
pub fn LoadMusicStreamFromMemory(fileType: impl AsRef<str>, data: *const c_uchar, dataSize: i32)->Music{unsafe{raw::LoadMusicStreamFromMemory(CString::new(fileType.as_ref()).unwrap().as_ptr(), data, dataSize)}}
pub fn IsMusicValid(music: Music)->bool{unsafe{raw::IsMusicValid(music)}}
pub fn UnloadMusicStream(music: Music){unsafe{raw::UnloadMusicStream(music)}}
pub fn PlayMusicStream(music: Music){unsafe{raw::PlayMusicStream(music)}}
pub fn IsMusicStreamPlaying(music: Music)->bool{unsafe{raw::IsMusicStreamPlaying(music)}}
pub fn UpdateMusicStream(music: Music){unsafe{raw::UpdateMusicStream(music)}}
pub fn StopMusicStream(music: Music){unsafe{raw::StopMusicStream(music)}}
pub fn PauseMusicStream(music: Music){unsafe{raw::PauseMusicStream(music)}}
pub fn ResumeMusicStream(music: Music){unsafe{raw::ResumeMusicStream(music)}}
pub fn SeekMusicStream(music: Music, position: f32){unsafe{raw::SeekMusicStream(music, position)}}
pub fn SetMusicVolume(music: Music, volume: f32){unsafe{raw::SetMusicVolume(music, volume)}}
pub fn SetMusicPitch(music: Music, pitch: f32){unsafe{raw::SetMusicPitch(music, pitch)}}
pub fn SetMusicPan(music: Music, pan: f32){unsafe{raw::SetMusicPan(music, pan)}}
pub fn GetMusicTimeLength(music: Music)->f32{unsafe{raw::GetMusicTimeLength(music)}}
pub fn GetMusicTimePlayed(music: Music)->f32{unsafe{raw::GetMusicTimePlayed(music)}}
pub fn LoadAudioStream(sampleRate: u32, sampleSize: u32, channels: u32)->AudioStream{unsafe{raw::LoadAudioStream(sampleRate, sampleSize, channels)}}
pub fn IsAudioStreamValid(stream: AudioStream)->bool{unsafe{raw::IsAudioStreamValid(stream)}}
pub fn UnloadAudioStream(stream: AudioStream){unsafe{raw::UnloadAudioStream(stream)}}
pub fn UpdateAudioStream(stream: AudioStream, data: *const c_void, frameCount: i32){unsafe{raw::UpdateAudioStream(stream, data, frameCount)}}
pub fn IsAudioStreamProcessed(stream: AudioStream)->bool{unsafe{raw::IsAudioStreamProcessed(stream)}}
pub fn PlayAudioStream(stream: AudioStream){unsafe{raw::PlayAudioStream(stream)}}
pub fn PauseAudioStream(stream: AudioStream){unsafe{raw::PauseAudioStream(stream)}}
pub fn ResumeAudioStream(stream: AudioStream){unsafe{raw::ResumeAudioStream(stream)}}
pub fn IsAudioStreamPlaying(stream: AudioStream)->bool{unsafe{raw::IsAudioStreamPlaying(stream)}}
pub fn StopAudioStream(stream: AudioStream){unsafe{raw::StopAudioStream(stream)}}
pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32){unsafe{raw::SetAudioStreamVolume(stream, volume)}}
pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32){unsafe{raw::SetAudioStreamPitch(stream, pitch)}}
pub fn SetAudioStreamPan(stream: AudioStream, pan: f32){unsafe{raw::SetAudioStreamPan(stream, pan)}}
pub fn SetAudioStreamBufferSizeDefault(size: i32){unsafe{raw::SetAudioStreamBufferSizeDefault(size)}}
