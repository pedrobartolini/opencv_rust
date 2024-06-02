
# rust_opencv

This project illustrates how to use opencv rust bindings and winapi

## Requirements
Opencv 4.x installed

### Env variables

- OPENCV_LINK_LIBS Comma separated list of library names to link to. .lib, .so or .dylib extension is optional. If you specify the ".framework" extension then build script will link a macOS framework instead of plain shared library. E.g. "opencv_world411".

- OPENCV_LINK_PATHS Comma separated list of paths to search for libraries to link. E.g. "C:\tools\opencv\build\x64\vc15\lib". The path list can start with '+', see OPENCV_LINK_LIBS for a detailed explanation (e.g. "+/usr/local/lib").

- OPENCV_INCLUDE_PATHS Comma separated list of paths to search for system include files during compilation. E.g. "C:\tools\opencv\build\include". One of the directories specified therein must contain "opencv2/core/version.hpp" or "core/version.hpp" file, it's used to detect the version of the headers. The path list can start with '+', see OPENCV_LINK_LIBS for a detailed explanation (e.g. "+/opt/cuda/targets/x86_64-linux/include/").

