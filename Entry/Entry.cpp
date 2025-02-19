#include <windows.h>
#include <tlhelp32.h>
#include <iostream>
#include <string>
#include <vector>

std::string WStringToString(const std::wstring& wstr, UINT codePage = CP_UTF8) {
    if (wstr.empty()) return std::string();

    // Determine the required buffer size
    int sizeNeeded = WideCharToMultiByte(
        codePage,                // Code page (e.g., CP_UTF8 for UTF-8, CP_ACP for ANSI)
        0,                      // Flags
        wstr.c_str(),            // Input wide string
        (int)wstr.size(),        // Length of the wide string
        nullptr,                 // Output buffer (nullptr to get the required size)
        0,                       // Output buffer size (0 to get the required size)
        nullptr,                 // Default character (used if a character cannot be converted)
        nullptr                  // Flag indicating whether a default character was used
    );

    if (sizeNeeded == 0) {
        throw std::runtime_error("WideCharToMultiByte failed");
    }

    // Allocate a buffer for the narrow string
    std::vector<char> buffer(sizeNeeded);

    // Perform the conversion
    int result = WideCharToMultiByte(
        codePage,
        0,
        wstr.c_str(),
        (int)wstr.size(),
        buffer.data(),
        sizeNeeded,
        nullptr,
        nullptr
    );

    if (result == 0) {
        throw std::runtime_error("WideCharToMultiByte failed");
    }

    // Construct and return the narrow string
    return std::string(buffer.data(), buffer.size());
}

// Function to check if a process is running by name
bool IsProcessRunning(const std::string& processName) {
    HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (hSnapshot == INVALID_HANDLE_VALUE) {
        std::cerr << "Failed to create process snapshot." << std::endl;
        return false;
    }

    PROCESSENTRY32 pe;
    pe.dwSize = sizeof(PROCESSENTRY32);

    if (Process32First(hSnapshot, &pe)) {
        do {
            // Convert the process name to a std::string for comparison
            std::wstring currentProcessNameWide(pe.szExeFile);
            std::string currentProcessName = WStringToString(currentProcessNameWide);
            if (processName == currentProcessName) {
                CloseHandle(hSnapshot);
                return true;
            }
        } while (Process32Next(hSnapshot, &pe));
    }

    CloseHandle(hSnapshot);
    return false;
}

// Function to start a process
bool StartProcess(const std::string& path) {
    STARTUPINFOA si = { sizeof(si) };
    PROCESS_INFORMATION pi;

    if (CreateProcessA(
        nullptr,                   // No module name (use command line)
        const_cast<LPSTR>(path.c_str()), // Command line
        nullptr,                   // Process handle not inheritable
        nullptr,                   // Thread handle not inheritable
        FALSE,                     // Set handle inheritance to FALSE
        0,                         // No creation flags
        nullptr,                   // Use parent's environment block
        nullptr,                   // Use parent's starting directory
        &si,                       // Pointer to STARTUPINFO structure
        &pi                        // Pointer to PROCESS_INFORMATION structure
    )) {
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
        return true;
    }
    else {
        std::cerr << "Failed to start process: " << path << std::endl;
        return false;
    }
}

int WinMain(
    HINSTANCE hInstance,
    HINSTANCE hPrevInstance,
    LPSTR     lpCmdLine,
    int       nShowCmd
) {
    // Paths to the server and client executables
    std::string serverPath = ".\\bin\\becks_server.exe";
    std::string clientPath = ".\\bin\\becks_client.exe";

    // Check if the server is running
    if (!IsProcessRunning("becks_server.exe")) {
        std::cout << "Starting becks_server..." << std::endl;
        if (!StartProcess(serverPath)) {
            std::cerr << "Failed to start becks_server." << std::endl;
            return 1;
        }
    }
    else {
        std::cout << "becks_server is already running." << std::endl;
    }

    // Start the client
    std::cout << "Starting becks_client..." << std::endl;
    if (!StartProcess(clientPath)) {
        std::cerr << "Failed to start becks_client." << std::endl;
        return 1;
    }

    std::cout << "Both processes started successfully." << std::endl;
    return 0;
}