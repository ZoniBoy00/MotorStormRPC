use std::io;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        
        // Add comprehensive metadata to help AV software recognize this as legitimate
        res.set("CompanyName", "ZoniBoy00");
        res.set("FileDescription", "Discord Rich Presence for MotorStorm Pacific Rift via RPCS3");
        res.set("FileVersion", "0.2.1.0");
        res.set("InternalName", "MotorStormRPC");
        res.set("LegalCopyright", "MIT License");
        res.set("OriginalFilename", "MotorStormRPC.exe");
        res.set("ProductName", "MotorStorm Discord RPC");
        res.set("ProductVersion", "0.2.1.0");
        
        // Set manifest to specify required execution level
        res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <assemblyIdentity
        version="0.2.1.0"
        processorArchitecture="*"
        name="MotorStormRPC"
        type="win32"
    />
    <description>Discord Rich Presence for MotorStorm Pacific Rift</description>
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="asInvoker" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
        <application>
            <!-- Windows 10 and Windows 11 -->
            <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
        </application>
    </compatibility>
</assembly>
"#);
        
        res.compile()?;
    }
    Ok(())
}
