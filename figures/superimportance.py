import numpy as np
import matplotlib.pyplot as plt
from matplotlib.collections import LineCollection
from colormath.color_objects import LabColor, sRGBColor
from colormath.color_conversions import convert_color
import os


C = 43.0
L = 60.
N = 8

anglestep = 2*np.pi/N
angles = np.arange(N)*anglestep



def DIN99_2_Lab(DIN99_ef):
    deg16 = np.deg2rad(16)
    s16 = np.sin(deg16)
    c16 = np.cos(deg16)

    e,f = DIN99_ef
    fk = f/0.7

    a = e*c16-fk*s16
    b = e*s16+fk*c16

    return a,b

def DIN99_2_sRGB(DIN99_ef):
    global L
    a,b = DIN99_2_Lab(DIN99_ef)
    
    lab = LabColor(L,a,b,illuminant='d65')
    sRGB : sRGBColor = convert_color(lab,target_cs=sRGBColor)
    for value in (sRGB.rgb_r,sRGB.rgb_b,sRGB.rgb_g):
        if value < 0.0 or value > 1.0:
            print("out of gamut")
            return (255,0,255)
    return sRGB.get_value_tuple()


def get_cols(e99,f99):
    return [DIN99_2_sRGB((e,f)) for e,f in zip(e99,f99) ]

def get_segs(e99,f99):
    segs = []
    for i in range(len(e99)-1):
        e,f = e99[i],f99[i]
        e2,f2 = e99[i+1],f99[i+1]
        segs.append(((e,f),(e2,f2)))
    return segs

def gradient_lc(e99,f99):
    lc = LineCollection(
        get_segs(e99,f99),
        colors = get_cols(e99,f99),
        linewidth=15)
    lc.set_capstyle("round")
    return lc


print(DIN99_2_sRGB((0,0)))





ax : plt.Axes = plt.gca()
for angle in angles:
    rs = np.linspace(0,C,12)
    e,f = rs*np.cos(angle),rs*np.sin(angle)
    ax.add_collection(gradient_lc(e,f))

theta = np.linspace(0,2*np.pi,N*50)
cx,cy = C*np.cos(theta),C*np.sin(theta)
ax.add_collection(gradient_lc(cx,cy))
ax.set_aspect('equal')
ax.set_axis_off()
plt.xlim(-C*1.3,C*1.3)
plt.ylim(-C*1.3,C*1.3)
plt.savefig(os.path.join(os.path.dirname(__file__),'wheel_euclidean.png'))



R_hyper =  28.6

true_circum = 2*np.pi * R_hyper *np.sinh(C/R_hyper)

amp = 3.6
rrs = C + amp * (1-np.cos(theta*N*2))
ccx,ccy = rrs*np.cos(theta),rrs*np.sin(theta)

pts = np.array([ccx,ccy]).T
seglengths = np.linalg.norm(np.roll(pts,1,axis=0)-pts,axis=-1)
print(seglengths.shape)
length = seglengths.sum()
print(f"True circ {true_circum:.2f}, current {length:.2f}")


plt.clf()
ax : plt.Axes = plt.gca()
for angle in angles:
    rs = np.linspace(0,C,12)
    e,f = rs*np.cos(angle),rs*np.sin(angle)
    ax.add_collection(gradient_lc(e,f))

wlc = LineCollection(get_segs(ccx,ccy),colors = get_cols(cx,cy),linewidth=15)
wlc.set_capstyle('round')
ax.add_collection(wlc)
ax.set_aspect('equal')
ax.set_axis_off()
plt.xlim(-C*1.3,C*1.3)
plt.ylim(-C*1.3,C*1.3)
plt.savefig(os.path.join(os.path.dirname(__file__),'wheel_wavy.png'))