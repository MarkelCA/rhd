# benchmarks
These benchmarks have been performed using [Valgrind](https://valgrind.org/) and their purpose is to measure the memory usage of the hex dumper.

The input file it's a 58MB zip file containing [the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).

To display them in a more readable way, use the `ms_print` tool that comes with Valgrind.
Example:
```
ms_print massif.out.22804
```
