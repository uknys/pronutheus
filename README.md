# Pronutheus

Bad pun between ONU and Prometheus

In the long term, it should be an exporter for a lantiq GPON (like the Nokia G-010S-P or FS GPON-ONU-34-20BI).

It uses the low level ioctl api to do the query to the ONU Driver (sources on base_sources/gpon_onu_drv-4.5.0.tar.gz)

I'm using a FS GPON-ONU-34-20BI for all the testing.

For now it only shows the equivalent of `onu ploamsg` but it'll become a full exporter !

## Sources
https://github.com/kbridgers/VOLTE4GFAX/tree/master/

Thanks to everyone who helped :)