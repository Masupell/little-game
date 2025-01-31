shader_type canvas_item;

//uniform vec2 size = vec2(108.8, 108.8);
vec3 palette(float t)
{
	vec3 a = vec3(0.5, 0.5, 0.5);
	vec3 b = vec3(0.5, 0.5, 0.5);
	vec3 c = vec3(1.0, 1.0, 1.0);
	vec3 d = vec3(0.263, 0.416, 0.557);
	return a + b*cos(6.28318*(c*t+d));
}

void fragment() {
	vec4 input_color = texture(TEXTURE, UV);
	vec2 uv = UV;
	uv.y = 1.0 - uv.y;
	uv = (uv - 0.5) * 2.0;
	//uv.x *= size.x/size.y; // To avoid stretching, but I cant get the size


	vec2 uv0 = uv;
	vec3 finalcolor = vec3(0.0);

	for (float i = 0.0; i < 4.0; i++)
	{
		uv = fract(uv*1.5)-0.5;

		float d = length(uv) * exp(-length(uv0));
		vec3 col = palette(length(uv0) + TIME*0.4 + i*0.4);

		d = sin(d*8.0 + TIME)/8.0;
		d = abs(d);

		d = pow(0.01/d, 1.2);
		finalcolor += col * d;
	}

	COLOR = vec4(finalcolor, input_color.a);
}

void vertex() {
	// Called for every vertex the material is visible on.
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
