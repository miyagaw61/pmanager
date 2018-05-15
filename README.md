# pmanager - Process manager witten in Rust

This manages a heavy process.
You can start/kill any process automatically.

## Install

```
$ cargo install --git https://github.com/miyagwa61/pmanager
```

## Usgae

```
$ pmanager [process-num] [start-command] [kill-command]
```

### process-num

```
$ vmstat
procs -----------memory---------- ---swap-- -----io---- -system-- ------cpu-----
 r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st
 4  0  41336 2951864 402508 2898208    0    2     5    18   34   89  3  0 97  0  0
 ```

process-num is this `r=4`

### start-command

process start command

(e.g.)
- `sudo systemctl start foo`
- `./bar`

### kill-command

process kill command

(e.g.)
- `sudo systemctl stop foo`
- `sudo pkill foo`

## Cron

You can register this to crontab.

```
$ cat ~/for_cron.sh
/path/to/pmanager 6 "export PATH=$PATH:/path/to; foo.sh" "sudo pkill -9 foo.sh"
```

```
$ crontab -l
* * * * * for i in `seq 0 5 59`;do (sleep ${i}; ~/for_cron.sh) & done;
```
