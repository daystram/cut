import Prism from "prismjs";
import { languages } from "@/constants";

export function highlighter(language: string): Function {
  return (code: string): string => {
    if (language === "Plaintext")
      return code
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
    return Prism.highlight(
      code,
      languages[language].grammar,
      languages[language].language
    );
  };
}
