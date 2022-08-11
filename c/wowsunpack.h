#ifndef _WOWS_UNPACK_H_
#define _WOWS_UNPACK_H_

/// Decode GameParams.data to GameParams.json
/// path - a string pointing to the GameParams.data file
/// compact - a boolean indicating the format mode
int cwowsunpack(char*, int);

#endif
