declare module 'mouse_listener' {
    export function hello(): string;
    
    export function parseAsync(input: string, callback: (error: Error | null, result: string) => void): void;
    
    export function listenMouse(callback: (error: Error | null, result: string) => void): void;
  }