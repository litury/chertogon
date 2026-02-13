interface SectionTitleProps {
  children: string;
  subtitle?: string;
}

export function SectionTitle({ children, subtitle }: SectionTitleProps) {
  return (
    <div className="mb-16 text-center">
      <h2 className="font-display text-4xl font-bold tracking-wide text-rune-blue md:text-5xl">
        {children}
      </h2>
      {subtitle && (
        <p className="mt-4 text-lg text-fog">{subtitle}</p>
      )}
      <div className="mx-auto mt-6 h-px w-24 bg-gradient-to-r from-transparent via-rune-blue to-transparent" />
    </div>
  );
}
