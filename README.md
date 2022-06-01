# Pronutheus

Bad pun between ONU and Prometheus

In the long term, it should be an exporter for a lantiq GPON (like the Nokia G-010S-P or FS GPON-ONU-34-20BI).

It uses the low level ioctl api to do the query to the ONU Driver (sources on base_sources/gpon_onu_drv-4.5.0.tar.gz)

I'm using a FS.com GPON-ONU-34-20BI for all the testing.

For now it only shows the equivalent of `onu ploamsg` but it'll become a full exporter !

## Compile

You can use the container that is included in .devcontainer and use `xargo build --release` to build the project.

## How to use it

```
$ export SSHPASS='7sp!lwUBz1'

$ sshpass -e scp -o StrictHostKeyChecking=no -o KexAlgorithms=+diffie-hellman-group1-sha1 -o HostKeyAlgorithms=+ssh-rsa target/mips-unknown-linux-uclibc/release/pronutheus ONTUSER@192.168.1.10:/home/ONTUSER/

$ sshpass -e ssh ONTUSER@192.168.1.10 -o StrictHostKeyChecking=no -o KexAlgorithms=+diffie-hellman-group1-sha1 -o HostKeyAlgorithms=+ssh-rsa '/home/ONTUSER/pronutheus'
```

*Warning : The password is only for the FS.com GPON-ONU-34-20BI.*

## Sources
https://github.com/kbridgers/VOLTE4GFAX/tree/master/

Thanks to everyone who helped :)