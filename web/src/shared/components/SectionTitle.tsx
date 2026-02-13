interface SectionTitleProps {
  children: string;
  subtitle?: string;
}

export function SectionTitle({ children, subtitle }: SectionTitleProps) {
  return (
    <div className="mb-16 text-center">
      <h2 className="font-display text-4xl font-bold tracking-wide text-gold md:text-5xl">
        {children}
      </h2>
      {subtitle && (
        <p className="mt-4 text-lg text-parchment">{subtitle}</p>
      )}
      <img
        src="/images/stone_divider.png"
        alt=""
        className="mx-auto mt-6 h-4 w-48 object-cover opacity-50"
      />
    </div>
  );
}
