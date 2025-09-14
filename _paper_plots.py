#!/usr/bin/env python
# coding: utf-8

# In[1]:


import os
import numpy             as np
import matplotlib.pyplot as plt
import pandas            as pd 
from matplotlib.ticker import (AutoMinorLocator, MultipleLocator)


folder = './plots/paper/'
if not os.path.exists(folder):
    os.makedirs(folder)


# In[2]:


sd_jwt_color = '#F2BB66'
csd_jwt_color= '#779ECB'


# In[3]:


sd_jwt_marker = 'D'
csd_jwt_marker = 's'


# In[4]:


marker_range_10 = range(0,10,1)
marker_range_100 = range(0,100,10)


# In[5]:


df = pd.read_csv("./csv_dir/vc_issuance_duration.csv")
fig, ax = plt.subplots()
x = range(1,101,1)

ax.plot(x, df['CSD-JWT'] / 1000, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_100, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1000, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_100, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_yscale("log")

ax.set_xlabel("Number of Claims")
ax.set_ylabel("Latency (ms)")

ax.legend(loc="upper left", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 100)
plt.ylim(10**-2, 10**2)

plt.grid(True, which='both', linestyle=':', linewidth=0.5)
plt.savefig(f"{folder}/VC issuance latency.pdf", dpi=600, format='pdf')
plt.show()


# In[6]:


df = pd.read_csv("./csv_dir/vc_jwt_length.csv")
fig, ax = plt.subplots()
x = range(1,101,1)

ax.plot(x, df['CSD-JWT'] / 1024, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_100, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1024, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_100, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))

ax.yaxis.set_major_locator(MultipleLocator(1))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_xlabel("Number of Claims")
ax.set_ylabel("Storage Requirement (KB)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')

ax.legend(loc="upper left", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 100)
plt.ylim(0, 20)

plt.savefig(f"{folder}/VC storage requirement.pdf", dpi=600, format='pdf')
plt.show()


# In[7]:


df = pd.read_csv("./csv_dir/vc_verification_duration.csv")
fig, ax = plt.subplots()
x = range(1,101,1)

ax.plot(x, df['CSD-JWT'] / 1000, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_100, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1000, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_100, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_yscale("log")

ax.set_xlabel("Number of Claims")
ax.set_ylabel("Latency (ms)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')

ax.legend(loc="upper left", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 100)
plt.ylim(10**-2, 10**2)

plt.savefig(f"{folder}/VC verification latency.pdf", dpi=600, format='pdf')
plt.show()


# In[8]:


df = pd.read_csv("./csv_dir/10_vp_issuance_duration.csv")
fig, ax = plt.subplots()
x = range(1,101,10)

ax.plot(x, df['CSD-JWT'] / 1000, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1000, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_10, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_yscale("log")

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("Latency (ms)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')

ax.legend(loc="right", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 91)
plt.ylim(10**-2, 10**-1)

plt.savefig(f"{folder}/10 VP issuance latency.pdf", dpi=600, format='pdf')
plt.show()


# In[9]:


df = pd.read_csv("./csv_dir/100_vp_issuance_duration.csv")
fig, ax = plt.subplots()
x = range(1,101,10)

ax.plot(x, df['CSD-JWT'] / 1000, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1000, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_10, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))
ax.set_yscale("log")

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("Latency (ms)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')

ax.legend(loc="right", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 91)
plt.ylim(10**-2, 10**0)

plt.savefig(f"{folder}/100 VP issuance latency.pdf", dpi=600, format='pdf')
plt.show()


# In[10]:


df = pd.read_csv("./csv_dir/10_vp_jwt_length.csv")
fig, ax = plt.subplots()
x = range(1,11,1)

ax.plot(x, df['CSD-JWT'] / 1024, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1024, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, (df['SD-JWT'] - df['CSD-JWT']) / 1024, marker='+', label='CSD-JWT Size Reduction w.r.t. SD-JWT', color='#666666', linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(1))
ax.yaxis.set_major_locator(MultipleLocator(0.5))

ax.xaxis.set_minor_locator(AutoMinorLocator(5))
ax.yaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("Size (KB)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')    

ax.legend(loc='lower right', fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 10)
plt.ylim(0, 3)
plt.savefig(f"{folder}/10 Claims VP Size.pdf", dpi=600, format='pdf')
plt.show()


# In[11]:


df = pd.read_csv("./csv_dir/100_vp_jwt_length.csv")
fig, ax = plt.subplots()
x = range(1,101,10)

ax.plot(x, df['CSD-JWT'] / 1024, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1024, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, (df['SD-JWT'] - df['CSD-JWT']) / 1024, marker='<', label='CSD-JWT Size Reduction w.r.t. SD-JWT', color='#C23B22', linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.yaxis.set_major_locator(MultipleLocator(1))

ax.xaxis.set_minor_locator(AutoMinorLocator(5))
ax.yaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("Size (KB)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')    

ax.legend(loc='upper left', fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 91)
plt.ylim(0, 20)

plt.savefig(f"{folder}/100 Claims VP Size.pdf", dpi=600, format='pdf')
plt.show()


# In[12]:


dfs = []

for i in range(1,11):
    dfs.append(pd.read_csv("./csv_dir/"+ str(i) + "0_vp_jwt_length.csv"))
    
for df in dfs: 
    df['CSD-JWT'] = df['CSD-JWT'] / 1024
    df['SD-JWT'] = df['SD-JWT'] / 1024

fig, ax = plt.subplots()
x = range(1, 101, 10)

ax.plot(x, dfs[9]['SD-JWT'] - dfs[9]['CSD-JWT'], color='#C23B22', marker='<', label='100 Claims')
ax.plot(x, dfs[8]['SD-JWT'] - dfs[8]['CSD-JWT'], color='#F49AC2', marker='>', label='90 Claims')
ax.plot(x, dfs[7]['SD-JWT'] - dfs[7]['CSD-JWT'], color='#FFB347', marker='o', label='80 Claims')
ax.plot(x, dfs[6]['SD-JWT'] - dfs[6]['CSD-JWT'], color='#B19CD9', marker='*', label='70 Claims')
ax.plot(x, dfs[5]['SD-JWT'] - dfs[5]['CSD-JWT'], color='#00CED1', marker='x', label='60 Claims')
ax.plot(x, dfs[4]['SD-JWT'] - dfs[4]['CSD-JWT'], color='#779ECB', marker='v', label='50 Claims')
ax.plot(x, dfs[3]['SD-JWT'] - dfs[3]['CSD-JWT'], color='#AEC6CF', marker='p', label='40 Claims')
ax.plot(x, dfs[2]['SD-JWT'] - dfs[2]['CSD-JWT'], color='#C19A6B', marker='.', label='30 Claims')
ax.plot(x, dfs[1]['SD-JWT'] - dfs[1]['CSD-JWT'], color='#836953', marker='1', label='20 Claims')
ax.plot(x, dfs[0]['SD-JWT'] - dfs[0]['CSD-JWT'], color='#666666', marker='+', label='10 Claims')

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.yaxis.set_major_locator(MultipleLocator(1))

ax.xaxis.set_minor_locator(AutoMinorLocator(5))
ax.yaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("CSD-JWT Size Reduction w.r.t. SD-JWT (KB)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')    


ax.legend(loc='upper right', fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 91)
plt.ylim(0, 12)

plt.savefig(f"{folder}/VP size varying disclosed claims.pdf", dpi=600, format='pdf')
plt.show()


# In[13]:


df = pd.read_csv("./csv_dir/10_vp_verification_duration.csv")
fig, ax = plt.subplots()
x = range(1,101,10)

ax.plot(x, df['CSD-JWT'] / 1000, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1000, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_10, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))

ax.set_yscale("log")

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("Latency (ms)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')

ax.legend(loc="right", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 91)
plt.ylim(10**-2, 10**1)

plt.savefig(f"{folder}/10 Claims VP verification latency.pdf", dpi=600, format='pdf')
plt.show()


# In[14]:


df = pd.read_csv("./csv_dir/100_vp_verification_duration.csv")
fig, ax = plt.subplots()
x = range(1,101,10)

ax.plot(x, df['CSD-JWT'] / 1000, label='CSD-JWT', color=csd_jwt_color, marker=csd_jwt_marker, markevery=marker_range_10, linewidth=2)
ax.plot(x, df['SD-JWT'] / 1000, label='SD-JWT', color=sd_jwt_color, marker=sd_jwt_marker, markevery=marker_range_10, linewidth=2)

ax.xaxis.set_major_locator(MultipleLocator(10))
ax.xaxis.set_minor_locator(AutoMinorLocator(5))
ax.set_yscale("log")

ax.set_xlabel("Number of Disclosed Claims")
ax.set_ylabel("Latency (ms)")

ax.grid(which='major', color='#EEEEEE', linestyle='solid')
ax.grid(which='minor', color='#EEEEEE', linestyle='solid')

ax.legend(loc="right", fancybox=True, framealpha=0.4, prop={'size': 9})

plt.xlim(1, 91)
plt.ylim(10**-2, 10**2)

plt.savefig(f"{folder}/100 Claims VP verification latency.pdf", dpi=600, format='pdf')
plt.show()

