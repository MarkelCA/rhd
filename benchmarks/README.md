# benchmarks
The input file for this benchmarks it's a 58MB zip file containing [the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
```bash
du ~/awscliv2.zip
58708	/home/markel/awscliv2.zip
```

## CPU Info
Before introducing the benchmarks, here is the CPU information of the machine where the benchmarks have been performed.
```
Architecture:            x86_64
  CPU op-mode(s):        32-bit, 64-bit
  Address sizes:         39 bits physical, 48 bits virtual
  Byte Order:            Little Endian
CPU(s):                  8
  On-line CPU(s) list:   0-7
Vendor ID:               GenuineIntel
  Model name:            11th Gen Intel(R) Core(TM) i5-1135G7 @ 2.40GHz
    CPU family:          6
    Model:               140
    Thread(s) per core:  2
    Core(s) per socket:  4
    Socket(s):           1
    Stepping:            1
    CPU max MHz:         4200,0000
    CPU min MHz:         400,0000
```
## Time
```bash
rhd ~/awscliv2.zip  3,72s user 2,00s system 99% cpu 5,726 total
```
Comparison with `xxd`:
```bash
xxd ~/awscliv2.zip  1,59s user 3,60s system 93% cpu 5,531 total
```
## Memory
These benchmarks have been performed using [Valgrind](https://valgrind.org/) and their purpose is to measure the memory usage of the hex dumper.

To display them in a more readable way, use the `ms_print` tool that comes with Valgrind.
```bash
ms_print massif.out.71203
```
Output:
```
--------------------------------------------------------------------------------
Command:            rhd /home/markel/awscliv2.zip
Massif arguments:   (none)
ms_print arguments: massif.out.71203
--------------------------------------------------------------------------------


    KB
26.34^                                                                     @
     |#:@@ ::        ::   :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  ::        :    :     :  ::  :     :     :      ::@      :@ ::: :@ :
     |#:@  :::  ::  ::  : :  :::: :::  :::   :::   ::::   ::@   ::::@ :::::@ :
     |#:@  :::  :   ::  : :  :: : :::  :::   :::   :: :   ::@   ::::@ :::::@ :
     |#:@  :::  :   ::  : :  :: : :::  :::   :::   :: :   ::@   ::::@ :::::@ :
     |#:@  :::  :   ::  : :  :: : :::  :::   :::   :: :   ::@   ::::@ :::::@ :
     |#:@  :::  :   ::  : :  :: : :::  :::   :::   :: :   ::@   ::::@ :::::@ :
     |#:@  :::  : @ ::  : :  :: :@:::  ::::  ::::: :: :@  ::@ ::::::@ :::::@ :
     |#:@  :::  : @ ::  : :  :: :@:::  ::::  ::::: :: :@  ::@ ::::::@ :::::@ :
     |#:@ ::::::: @:::  : ::::: :@:::::::::  ::::: :: :@: ::@:::::::@ :::::@ :
     |#:@ ::::::: @::: :: :: :: :@:::::::::::::::: :: :@: ::@:::::::@::::::@ :
     |#:@ ::::::: @::: ::::: :: :@:::::::::: :::::::: :@::::@:::::::@::::::@::
   0 +----------------------------------------------------------------------->Gi
     0                                                                   40.63

Number of snapshots: 76
 Detailed snapshots: [1 (peak), 4, 12, 24, 46, 52, 62, 72]

--------------------------------------------------------------------------------
  n        time(i)         total(B)   useful-heap(B) extra-heap(B)    stacks(B)
--------------------------------------------------------------------------------
  0              0                0                0             0            0
  1      3,381,114           26,800           26,734            66            0
```
