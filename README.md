# dyn

algorithms and some other ramdom stuff

## run hello world in assembly

- install the assembler/linker

```sh
sudo apt install gcc
```

- run assembly

```sh
as asem.s -o asem.o
```

- run the linker

```sh
gcc -o asem asem.o -nostdlib -static
```

- execute the program

```sh
./asem
```
