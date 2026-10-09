// Official DesktopAcrylicController bridge. All calls belong to one UI thread.
#include <windows.h>
#include <dwmapi.h>
#include <DispatcherQueue.h>
#include <windows.ui.composition.interop.h>
#include <winrt/Windows.Foundation.h>
#include <winrt/Windows.System.h>
#include <winrt/Windows.UI.Composition.h>
#include <winrt/Windows.UI.Composition.Desktop.h>
#include <winrt/Microsoft.UI.Composition.SystemBackdrops.h>
#include <MddBootstrap.h>
#include <memory>
#include <string>
#include <sstream>
#include <cstring>
#include <cmath>

using namespace winrt;
using namespace Windows::UI::Composition;
using namespace Microsoft::UI::Composition::SystemBackdrops;
struct Context {
    HWND hwnd{};
    Compositor compositor{nullptr};
    Windows::UI::Composition::Desktop::DesktopWindowTarget target{nullptr};
    ContainerVisual root{nullptr};
    DesktopAcrylicController controller{nullptr};
    SystemBackdropConfiguration config{nullptr};
};
static bool bootstrapped = false;
static bool apartment_initialized = false;
static Windows::System::DispatcherQueueController queue{nullptr};
static std::string last_error;

extern "C" __declspec(dllexport) int __stdcall OfficialCreate(HWND hwnd, void** output) {
    try {
        if (!bootstrapped) {
            PACKAGE_VERSION minimum{}; minimum.Version=0x0002000500010000ULL;
            check_hresult(MddBootstrapInitialize2(0x00020005, L"", minimum, MddBootstrapInitializeOptions_None));
            bootstrapped = true;
        }
        if(!apartment_initialized){init_apartment(apartment_type::single_threaded);apartment_initialized=true;}
        if (!DesktopAcrylicController::IsSupported()) { last_error="DesktopAcrylicController unsupported"; return E_NOTIMPL; }
        auto ctx = std::make_unique<Context>(); ctx->hwnd=hwnd;
        if (!Windows::System::DispatcherQueue::GetForCurrentThread()) {
            DispatcherQueueOptions options{sizeof(DispatcherQueueOptions), DQTYPE_THREAD_CURRENT, DQTAT_COM_NONE};
            check_hresult(CreateDispatcherQueueController(options, reinterpret_cast<ABI::Windows::System::IDispatcherQueueController**>(put_abi(queue))));
        }
        BOOL enabled=TRUE; check_hresult(DwmSetWindowAttribute(hwnd,17,&enabled,sizeof(enabled)));
        ctx->compositor = Compositor();
        auto interop = ctx->compositor.as<ABI::Windows::UI::Composition::Desktop::ICompositorDesktopInterop>();
        check_hresult(interop->CreateDesktopWindowTarget(hwnd,FALSE,reinterpret_cast<ABI::Windows::UI::Composition::Desktop::IDesktopWindowTarget**>(put_abi(ctx->target))));
        ctx->root=ctx->compositor.CreateContainerVisual();ctx->root.RelativeSizeAdjustment({1,1});ctx->target.Root(ctx->root);
        ctx->controller=DesktopAcrylicController();ctx->config=SystemBackdropConfiguration();
        // A floating reader remains translucent while users work in another window.
        // The controller still respects system transparency/accessibility policies.
        ctx->config.IsInputActive(true);ctx->config.Theme(SystemBackdropTheme::Dark);
        ctx->controller.SetSystemBackdropConfiguration(ctx->config);
        const bool connected=ctx->controller.SetTarget(Microsoft::UI::WindowId{reinterpret_cast<uint64_t>(hwnd)},ctx->target);
        if (!connected) {last_error="SetTarget returned false";return E_FAIL;}
        ctx->config.IsInputActive(true);
        ctx->controller.SetSystemBackdropConfiguration(ctx->config);
        ctx->controller.TintColor(Windows::UI::Color{255,20,26,31});
        ctx->controller.FallbackColor(Windows::UI::Color{255,20,26,31});
        ctx->controller.TintOpacity(0);ctx->controller.LuminosityOpacity(0);
        *output=ctx.release();last_error.clear();return S_OK;
    } catch(hresult_error const& error) {last_error=to_string(error.message());return error.code();}
}
extern "C" __declspec(dllexport) int __stdcall OfficialSet(void* opaque,float tint,float luminosity) {
    try {
        if(!opaque||!std::isfinite(tint)||!std::isfinite(luminosity)||tint<0||tint>1||luminosity<0||luminosity>1)return E_INVALIDARG;
        auto ctx=static_cast<Context*>(opaque);
        ctx->config.IsInputActive(true);
        ctx->controller.SetSystemBackdropConfiguration(ctx->config);
        ctx->controller.TintOpacity(tint);ctx->controller.LuminosityOpacity(luminosity);
        return S_OK;
    } catch(hresult_error const& error) {last_error=to_string(error.message());return error.code();}
}
extern "C" __declspec(dllexport) int __stdcall OfficialInfo(void* opaque,char* output,int capacity) {
    std::ostringstream text;
    if(opaque){auto ctx=static_cast<Context*>(opaque);text<<"{\"tintOpacity\":"<<ctx->controller.TintOpacity()<<",\"luminosityOpacity\":"<<ctx->controller.LuminosityOpacity()<<",\"state\":"<<static_cast<int>(ctx->controller.State())<<",\"inputActive\":"<<(ctx->config.IsInputActive()?"true":"false")<<",\"foreground\":"<<(GetForegroundWindow()==ctx->hwnd?"true":"false")<<"}";}
    else {text<<"{\"error\":\"";for(char ch:last_error){if(ch=='\\'||ch=='\"')text<<'\\';if(static_cast<unsigned char>(ch)>=32)text<<ch;}text<<"\"}";}
    auto value=text.str();if(capacity<=static_cast<int>(value.size()))return E_INVALIDARG;
    std::memcpy(output,value.c_str(),value.size()+1);return S_OK;
}
extern "C" __declspec(dllexport) void __stdcall OfficialClose(void* opaque) {
    auto ctx=std::unique_ptr<Context>(static_cast<Context*>(opaque));
    if(ctx&&ctx->controller)ctx->controller.Close();
}
