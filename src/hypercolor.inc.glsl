//(Lstar +16.)/116.
uniform float Ltilde;

vec2 hplane2poinc(float hx,float hy){
    float mult = 1.0/(hx*hx + (1.0+hy)*(1.0+hy));
    return vec2(mult*2.0*hx, mult*(hx*hx+hy*hy-1.0));
}

vec2 poinc2hplane(float px,float py){
    float mult = 1.0/(px*px + (1.-py)*(1.-py));

    return vec2(
        mult * 2.0 * px,
        mult * (1.0 - px*px - py*py)
    );
}


vec2 cdiv(vec2 w,vec2 z){
    float den = dot(z,z);
    return vec2(
        w.x * z.x + w.y * z.y,
        w.y * z.x - w.x * z.y
    )/den;
}


vec2 translate(vec2 h, float lambda){
    float ch = cosh(lambda);
    float sh = sinh(lambda);

    vec2 over = ch * h + vec2(sh,0.);
    vec2 under = sh * h + vec2(ch,0.);

    return cdiv(over,under);
}



vec2 rotate(vec2 v, float angle){
    float c = cos(angle);
    float s = sin(angle);

    return vec2(c*v.x + s*v.y, -s*v.x + c*v.y);
}


const float hyperR = 28.6;

//sine and cosine of 16°
const float s16 = 0.27563735582;
const float c16 = 0.96126169594;

vec2 poinc2DIN99(vec2 poinc){
    float prho = sqrt(dot(poinc,poinc));

    float geodesic_r = 2. * atanh(prho);

    float chroma99 = hyperR * geodesic_r;

    float G = (exp(chroma99*0.045) - 1.)/0.045;

    float theta = atan(poinc.y,poinc.x);

    float f = chroma99*cos(theta);
    float e = chroma99*sin(theta);

    return vec2(e,f);
}

vec2 DIN992Lab(vec2 d99){
    float e = d99.x;
    float f = d99.y;
    float fk = f/0.7;

    float a = e*c16 - fk*s16;
    float b = e*s16 + fk*c16;

    return vec2(a,b);
}






const float delta = 6./29.;
const float delta2 = delta*delta;

float finv(float tee){
    if (tee > delta)
        return tee*tee*tee;
    return 3.*delta2*(tee - 4./29.);
}

vec3 D65 = vec3(95.0489,100,108.8840);

vec3 Lab2XYZ(vec2 ab){
    float a = ab.x;
    float b = ab.y;

    vec3 arg = Ltilde + vec3(a/500.,0,-b/200.);
    return D65 * vec3(
        finv(arg.x),
        finv(arg.y),
        finv(arg.z)
    );

}



const mat3 XYZ_2_RGB = (mat3(
     3.2404542,-0.9692660, 0.0556434,
    -1.5371385, 1.8760108,-0.2040259,
    -0.4985314, 0.0415560, 1.0572252
));
const float SRGB_ALPHA = 0.055;
float linear_to_srgb(float channel) {
    if(channel <= 0.0031308)
        return 12.92 * channel;
    else
        return (1.0 + SRGB_ALPHA) * pow(channel, 1.0/2.4) - SRGB_ALPHA;
}
vec3 XYZ2sRGB(vec3 XYZ){
    vec3 rgb = XYZ_2_RGB * (XYZ/100.);

    vec3 srgb = vec3(
        linear_to_srgb(rgb.r),
        linear_to_srgb(rgb.g),
        linear_to_srgb(rgb.b)
    );

    

    return srgb;
}