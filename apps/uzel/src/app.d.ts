export {};

declare global {
  interface Window {
    NMPTrustedShellHost: {
      mount(
        surfaceId: string,
        surface: HTMLElement,
        configuration: {
          session: string;
          artifactBaseURL: string;
          artifactHTML: string;
          title: string;
          domains: string[];
        },
      ): boolean;
      receive(surfaceId: string, envelope: unknown): boolean;
      unmount(surfaceId: string): boolean;
    };
    __nmpTrustedShellMount(configuration: {
      session: string;
      artifactBaseURL: string;
      artifactHTML: string;
      title: string;
      domains: string[];
    }): boolean;
    __nmpTrustedShellReceive(envelope: unknown): boolean;
  }
}
