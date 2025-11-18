#include <Windows.h>

extern "C" {
	__declspec(dllexport) LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
		switch (uMsg) {
			case WM_DESTROY:
				PostQuitMessage(0);
				return 0;
			case WM_PAINT:
				{
					PAINTSTRUCT ps;
					HDC hdc = BeginPaint(hwnd, &ps);
					FillRect(hdc, &ps.rcPaint, (HBRUSH)(COLOR_WINDOW + 1));
					EndPaint(hwnd, &ps);
					return 0;
				}
		}

		return DefWindowProc(hwnd, uMsg, wParam, lParam);
	}
}

extern "C" {
	__declspec(dllexport) int WINAPI DLLWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
		const char CLASS_NAME[] = "Shipment Process";

		WNDCLASS wc = {};
		wc.lpfnWndProc = WindowProc;
		wc.hInstance = hInstance;
		wc.lpszClassName = CLASS_NAME;

		RegisterClass(&wc);

		HWND hwnd = CreateWindowEx(
				0, CLASS_NAME, "Shipment",
				WS_OVERLAPPEDWINDOW,
				CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
				NULL, NULL, hInstance, NULL);

		if (hwnd == NULL) {
			return 0;
		}

		// Call to show window - important to see the window!
		ShowWindow(hwnd, nCmdShow);

		// Msg loop
		MSG msg = {};
		while (GetMessage(&msg, NULL, 0, 0)) {
			TranslateMessage(&msg);
			DispatchMessage(&msg);
		}

		return 0;
	}
}

