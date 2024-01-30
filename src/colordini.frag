
in vec2 uvs;

layout (location = 0) out vec4 color;

uniform float dispx;
uniform float dispscale;


uniform float rot;
uniform int displaysurf;

void main()
{
    float hrho = uvs.x;
    float htheta = 1.0-uvs.y;
    vec2 hplane = vec2(hrho * sin(htheta), hrho * cos(htheta));

    hplane *= dispscale;
    hplane = translate(hplane, -dispx);

    vec2 poinc = hplane2poinc(hplane.x,hplane.y);

    poinc = rotate(poinc,rot);

    vec2 DIN99c = poinc2DIN99c(poinc);
    vec2 Lab = DIN99c2Lab(DIN99c);
    vec3 XYZ = Lab2XYZ(Lab);
    vec3 sRGB = XYZ2sRGB(XYZ);    


    if((sRGB.x<0.)||(sRGB.x>1.0)||(sRGB.y<0.)||(sRGB.y>1.0)||(sRGB.z<0.)||(sRGB.z>1.0))
    {
        if(displaysurf>0){
            float grid = mod(dot(floor(16.*poinc),vec2(1.,1.)),2.0);
            sRGB = (.5+grid*.2)*vec3(.5,.5,.5);
        }
        else
            discard;
    }

    vec3 col = sRGB;
    
    

    color = vec4(sRGB,1.0);
}   