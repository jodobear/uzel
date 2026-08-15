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
          manifestAuthor: string;
          dTag: string;
          aggregateHash: string;
          artifactDigest: string;
          onReady?: (surfaceId: string) => void;
          onError?: (surfaceId: string, detail: string) => void;
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
      onReady?: (surfaceId: string) => void;
      onError?: (surfaceId: string, detail: string) => void;
    }): boolean;
    __nmpTrustedShellReceive(envelope: unknown): boolean;
  }
}
