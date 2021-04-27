import java.io.BufferedReader;
import java.io.FileReader;
import java.io.FileWriter;

import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.util.headless.HeadlessScript;
import ghidra.framework.cmd.CompoundBackgroundCommand;
import ghidra.program.model.block.BasicBlockModel;

public class GetBBS extends HeadlessScript {

	@Override
	protected void run() throws Exception {
		var args = getScriptArgs();
		
		var compoundCmd = new CompoundBackgroundCommand("Disassemble extra code", true, true);
		
		if (args.length > 1) {
			var extraCodeFilename = args[1];
			try (var fileReader = new BufferedReader(new FileReader(extraCodeFilename))) {
				String line;
				while ((line = fileReader.readLine()) != null) {
					int offset = parseInt(line);

					var cmd = new DisassembleCommand(this.toAddr(offset), currentProgram.getMemory().getAllInitializedAddressSet(), true);
					compoundCmd.add(cmd);
				}
			}
		}
		if (!compoundCmd.applyTo(currentProgram)) {
			throw new Exception("Failed to disassemble extra code");
		}
		
		
		var bbm = new BasicBlockModel(currentProgram);
		var blocks = bbm.getCodeBlocks(monitor);
		var block = blocks.next();
		
		try (var fileWriter = new FileWriter(args[0])) {
			while (block != null) {
				fileWriter.write(String.format("%d\n", block.getMinAddress().getOffset()));
				block = blocks.next();
			}
		}
	}

}
