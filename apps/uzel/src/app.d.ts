export {};

declare global {
  interface Window {
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
