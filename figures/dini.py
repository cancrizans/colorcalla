import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d import Axes3D  
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')



us = np.linspace(-8*np.pi,8*np.pi,240)
vs = np.linspace(0.001,np.pi/2,32)

# u,v = np.meshgrid(us,vs)


# b = 0.2

def dini(u,v):
    b=0.5
    x = np.cos(u)*np.sin(v)
    y = np.sin(u)*np.sin(v)
    z = np.cos(v) + np.log(np.tan(v/2)) + b*u
    return x,y,z

x,y,z = dini(*np.meshgrid(us,vs))
ax.plot_surface(-z,y,x,color='white')

for u_off in us:
    u = -2 *np.log(np.sin(vs/2)) + u_off
    x,y,z = dini(u,vs)
    ax.plot(-z,y,x,c='black')
    u = -2 *np.log(np.cos(vs/2)) + u_off
    x,y,z = dini(u,vs)
    ax.plot(-z,y,x,c='black')





s = 4.
ax.scatter((-s,s),(-s,s),(-s,s))

ax.set_axis_off()

plt.show()
