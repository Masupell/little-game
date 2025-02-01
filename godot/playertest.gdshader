shader_type canvas_item;

uniform vec2 size;
uniform float frame : hint_range(0.0, 10.0, 1.0);

void fragment()
{
	vec4 input_color = texture(TEXTURE, UV);
	
	vec2 uv = vec2(UV.x*10.- frame, UV.y);
	uv.y = 1.0 - uv.y;
	uv = (uv - 0.5) * 2.0;
	uv.x *= size.x/size.y;
	
	float d = length(uv);
	d = sin(d*8.0+TIME)/8.0;
	d = abs(d);
	d = pow(0.02/d, 1.2);
	
	vec3 final = vec3(input_color.r+d, input_color.g+d, input_color.b+d);
	COLOR = vec4(final, input_color.a);
}