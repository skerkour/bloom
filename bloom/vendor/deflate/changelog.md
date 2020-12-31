<a name="0.8.6"></a>
### 0.8.6 (2020-07-06)


#### Bug Fixes

*   try to fix issues with sync flush behaviour ([6c97e514](6c97e514), closes [#48](48))
*   add #!forbid(unsafe_code) to crate root ([fcbe4206](fcbe4206))



<a name="0.8.5"></a>
### 0.8.5 (2020-07-04)


#### Bug Fixes

*   Avoid infinitely looping on sync flush with short buffer writers ([99a1a75f](99a1a75f), closes [#47](47))
*   Remove unsafe in write_length_rle ([77227c8b](77227c8b), closes [#46](46))



<a name="0.8.4"></a>
### 0.8.4 (2020-04-04)


#### Bug Fixes

*   Fix block size counter bug #44 (probably introduced in 1b70be)
that triggered a debug assertion and that could possibly in theory cause stored block to start at the wrong input position at a block split with low entropy data followed by uncompressible data.
