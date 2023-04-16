use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::*;

static TEXTURED_QUAD_VS: &'static str = include_str!("./textured-quad-vertex.glsl");
static TEXTURED_QUAD_FS: &'static str = include_str!("./textured-quad-fragment.glsl");

//static MESH_SKINNED_VS: &'static str = include_str!("./mesh-skinned-vertex.glsl");
//static MESH_SKINNED_FS: &'static str = include_str!("./mesh-skinned-fragment.glsl");

static MESH_NON_SKINNED_VS: &'static str = include_str!("./mesh-non-skinned-vertex.glsl");
static MESH_NON_SKINNED_FS: &'static str = include_str!("./mesh-non-skinned-fragment.glsl");

static WATER_VS: &'static str = include_str!("./water-vertex.glsl");
static WATER_FS: &'static str = include_str!("./water-fragment.glsl");

/// Powers retrieving and using our shaders
pub struct ShaderSystem {
    programs: HashMap<ShaderKind, Shader>,
    active_program: RefCell<ShaderKind>,
}

impl ShaderSystem {
    /// Create  a new ShaderSystem
    pub fn new(gl: &WebGl2RenderingContext) -> ShaderSystem {
        let mut programs = HashMap::new();

        let water_shader = Shader::new(&gl, WATER_VS, WATER_FS).unwrap();
        let non_skinned_shader =
            Shader::new(&gl, MESH_NON_SKINNED_VS, MESH_NON_SKINNED_FS).unwrap();
        //let skinned_mesh_shader = Shader::new(&gl, MESH_SKINNED_VS, MESH_SKINNED_FS).unwrap();
        let textured_quad_shader = Shader::new(&gl, TEXTURED_QUAD_VS, TEXTURED_QUAD_FS).unwrap();

        let active_program = RefCell::new(ShaderKind::TexturedQuad);
        gl.use_program(Some(&textured_quad_shader.program));

        programs.insert(ShaderKind::Water, water_shader);
        programs.insert(ShaderKind::NonSkinnedMesh, non_skinned_shader);
        //programs.insert(ShaderKind::SkinnedMesh, skinned_mesh_shader);
        programs.insert(ShaderKind::TexturedQuad, textured_quad_shader);

        ShaderSystem {
            programs,
            active_program,
        }
    }

    /// Get one of our Shader's
    pub fn get_shader(&self, shader_kind: &ShaderKind) -> Option<&Shader> {
        self.programs.get(shader_kind)
    }

    /// Use a shader program. We cache the last used shader program to avoid unnecessary
    /// calls to the GPU.
    pub fn use_program(&self, gl: &WebGl2RenderingContext, shader_kind: ShaderKind) {
        if *self.active_program.borrow() == shader_kind {
            return;
        }

        gl.use_program(Some(&self.programs.get(&shader_kind).unwrap().program));
        *self.active_program.borrow_mut() = shader_kind;
    }
}

/// Identifiers for our different shaders
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderKind {
    Water,
    NonSkinnedMesh,
    SkinnedMesh,
    TexturedQuad,
}

/// One per ShaderKind
pub struct Shader {
    pub program: WebGlProgram,
    uniforms: RefCell<HashMap<&'static str, WebGlUniformLocation>>,
    uniform_block_indexes: RefCell<HashMap<&'static str, u32>>,
}

impl Shader {
    /// Create a new Shader program from a vertex and fragment shader
    fn new(
        gl: &WebGl2RenderingContext,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<Shader, JsValue> {
        let vert_shader = compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)?;
        let frag_shader =
            compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        let uniforms = RefCell::new(HashMap::new());
        let uniform_block_indexes = RefCell::new(HashMap::new());

        Ok(Shader {
            program,
            uniforms,
            uniform_block_indexes,
        })
    }

    /// Get the location of a uniform.
    /// If this is our first time retrieving it we will cache it so that for future retrievals
    /// we won't need to query the shader program.
    pub fn get_uniform_location(
        &self,
        gl: &WebGl2RenderingContext,
        uniform_name: &'static str,
    ) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();

        if let Some(uni) = uniforms.get(uniform_name) {
            Some(uni.clone())
        } else {
            let uni = gl
                .get_uniform_location(&self.program, uniform_name)
                .expect(&format!(r#"Uniform '{}' not found"#, uniform_name));
            uniforms.insert(uniform_name, uni.clone());
            Some(uni)
        }
    }

    pub fn get_uniform_block_index(
        &self,
        gl: &WebGl2RenderingContext,
        uniform_block_name: &'static str,
    ) -> u32 {
        let mut uniform_blocks = self.uniform_block_indexes.borrow_mut();

        if let Some(idx) = uniform_blocks.get(uniform_block_name) {
            *idx
        } else {
            let idx = gl.get_uniform_block_index(&self.program, uniform_block_name);

            uniform_blocks.insert(uniform_block_name, idx);

            idx
        }
    }
}

/// Create a shader program using the WebGL APIs
fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

/// Link a shader program using the WebGL APIs
fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);

    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
