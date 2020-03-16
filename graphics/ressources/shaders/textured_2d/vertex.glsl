#version 140

in mat4 world_transformation;
in vec3 position;
in vec2 texture;

out vec2 v_tex_coords;
out vec3 v_position;

void main()
{
	v_tex_coords = texture;
	v_position = position;
	gl_Position = world_transformation*vec4(position, 1.0);
}
