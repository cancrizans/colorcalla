
in vec2 uvs;

layout (location = 0) out vec4 color;

uniform float tant;
uniform float dispx;
uniform float dispscale;
uniform float rot;

void main()
{
    vec2 poinc0 = (uvs-.5)*2.0;

    vec2 hplane = poinc2hplane(poinc0.x,poinc0.y);

    bool mask = (hplane.x > 0.0) && (hplane.y > tant*hplane.x);
    
    hplane *= dispscale;
    
    hplane = translate(hplane, -dispx);


    vec2 poinc = hplane2poinc(hplane.x,hplane.y);

    poinc = rotate(poinc, rot);

    

    if(dot(poinc,poinc)>1.)
        discard;

    vec2 DIN99 = poinc2DIN99(poinc);
    vec2 ab = DIN992Lab(DIN99);
    vec3 XYZ = Lab2XYZ(ab);
    vec3 sRGB = XYZ2sRGB(XYZ);

    if((sRGB.x<0.)||(sRGB.x>1.0)||(sRGB.y<0.)||(sRGB.y>1.0)||(sRGB.z<0.)||(sRGB.z>1.0))
        sRGB = vec3(.5,.5,.5);
    vec3 col = sRGB * (mask? 1.0:0.5);

    



    float grid = mod(dot(floor(16.*poinc),vec2(1.,1.)),2.0) + 0.0001*tant;
    vec3 checkr = grid * vec3(1.,1.,1.);

    color = vec4(col,1.0);
}   