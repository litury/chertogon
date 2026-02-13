import { Section, SectionTitle } from "../../shared/components";
import { SECTION_IDS } from "../../shared/constants";
import { CompatCheck } from "./parts/CompatCheck";
import { GameCanvas } from "./parts/GameCanvas";

export function Gameplay() {
  return (
    <Section id={SECTION_IDS.gameplay} className="bg-stone-floor/20">
      <SectionTitle subtitle="Прямо здесь, прямо сейчас">
        Испытай Силу
      </SectionTitle>

      <CompatCheck>
        <GameCanvas />
      </CompatCheck>
    </Section>
  );
}
