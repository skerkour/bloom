# call this script manually to regen bindings from a new version of libsodium
# make sure that you have up-to-date bindgen (at least 0.48.0 to avoid problems with alignments)

# a whitelist regex to generate entities
REGEX="(SODIUM|sodium|crypto|randombytes)_.*"

bindgen PATH_TO/libsodium-1.0.18/src/libsodium/include/sodium.h -o sodium_bindings.rs \
  --ctypes-prefix=libc --use-core \
  --generate=functions,types,vars \
  --whitelist-function=$REGEX \
  --whitelist-type=$REGEX \
  --whitelist-var=$REGEX
