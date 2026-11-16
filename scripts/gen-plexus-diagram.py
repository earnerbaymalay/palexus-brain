import sys
tools=[("CC","claude-code"),("CU","cursor"),("GE","gemini"),("WS","windsurf"),("CL","cline"),("RC","roo-code"),("GH","copilot"),("ZD","zed"),("AG","agy-cli"),("CX","codex"),("AI","aider"),("CN","continue"),("OC","opencode")]
W,H=1200,630; ink="#06080b"; line="#262c37"; acc="#1d9bf0"; dim="#8b949e"; txt="#d4dae2"
top,bot=70,H-70; n=len(tools); step=(bot-top)/(n-1)
bx,by=150,H/2; jx=470; ex=820
o=[f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="JetBrains Mono, ui-monospace, monospace">',
f'<rect width="{W}" height="{H}" fill="{ink}"/>']
# hairline grid
for x in range(0,W,48): o.append(f'<line x1="{x}" y1="0" x2="{x}" y2="{H}" stroke="#0b0e13"/>')
for y in range(0,H,48): o.append(f'<line x1="0" y1="{y}" x2="{W}" y2="{y}" stroke="#0b0e13"/>')
# brain node (square, registration marks)
s=56; o.append(f'<rect x="{bx-s}" y="{by-s}" width="{2*s}" height="{2*s}" fill="#0b0e13" stroke="{acc}" stroke-width="1.6"/>')
for cx,cy in [(bx-s,by-s),(bx+s,by-s),(bx-s,by+s),(bx+s,by+s)]:
    o.append(f'<path d="M{cx-6} {cy}H{cx+6}M{cx} {cy-6}V{cy+6}" stroke="{dim}" stroke-width="1"/>')
o.append(f'<text x="{bx}" y="{by-4}" fill="{txt}" font-size="15" font-weight="700" text-anchor="middle">BRAIN</text>')
o.append(f'<text x="{bx}" y="{by+18}" fill="{dim}" font-size="12" text-anchor="middle">~/AIBrain</text>')
# trunk
o.append(f'<path d="M{bx+s} {by}H{jx}" stroke="{acc}" stroke-width="3"/>')
o.append(f'<text x="{(bx+s+jx)/2}" y="{by-12}" fill="{dim}" font-size="12" text-anchor="middle">palexus project</text>')
for i,(mono,name) in enumerate(tools):
    y=top+i*step; d=abs(y-by)
    # 45-degree diverge then horizontal
    x45=jx+min(d,ex-jx-120)
    o.append(f'<path d="M{jx} {by}L{x45} {y}H{ex}" fill="none" stroke="{acc}" stroke-opacity="0.75" stroke-width="1.6"/>')
    r=16; hx=ex+r
    pts=" ".join(f"{hx+r*c:.1f},{y+r*s_:.1f}" for c,s_ in [(1,0),(.5,.866),(-.5,.866),(-1,0),(-.5,-.866),(.5,-.866)])
    o.append(f'<polygon points="{pts}" fill="#12161d" stroke="{acc}" stroke-width="1.2"/>')
    o.append(f'<text x="{hx}" y="{y+4}" fill="{txt}" font-size="11" font-weight="700" text-anchor="middle">{mono}</text>')
    o.append(f'<text x="{hx+r+14}" y="{y+4}" fill="{dim}" font-size="12">{name}</text>')
o.append(f'<circle cx="{jx}" cy="{by}" r="4" fill="{acc}"/>')
o.append(f'<text x="{jx-14}" y="{by+22}" fill="{acc}" font-size="12" text-anchor="end">the Plexus</text>')
o.append(f'<text x="48" y="44" fill="{txt}" font-size="13" letter-spacing="2">PALEXUS · ONE BRAIN. EVERY NERVE.</text>')
o.append(f'<text x="48" y="{H-28}" fill="{dim}" font-size="11">{n} adapters · one trunk · every edit routed through the Plexus</text>')
o.append('</svg>')
open(sys.argv[1],'w').write("\n".join(o))
